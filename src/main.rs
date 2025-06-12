use clap::{Parser, Subcommand};

mod commands;
mod ollama;

#[derive(Parser)]
#[command(name = "shc-tool")]
#[command(about = "System Health Check Tool")]
#[command(version = "0.1.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Zeigt lokale IP-Adressen und Gateway Informationen an
    Ip,
    /// Sendet einen Ping an ein Ziel
    Ping {
        /// Die Ziel-IP-Adresse oder der Hostname
        target: String,
    },
    /// Sammelt und analysiert System-Log-Dateien
    Logs {
        /// Erstelle ZIP-Archiv der Log-Dateien
        #[arg(long)]
        zip: bool,
        /// Analysiere Logs mit KI (Ollama)
        #[arg(long)]
        analyze: bool,
        /// Frage für die KI-Analyse
        #[arg(long)]
        query: Option<String>,
        /// Anzahl der zu analysierenden Zeilen (Standard: 200)
        #[arg(long, default_value = "200")]
        lines: usize,
        /// Ollama-Modell (Standard: gemma2:2b)
        #[arg(long)]
        model: Option<String>,
        /// Spezifischer Pfad zur Log-Datei
        #[arg(long)]
        file: Option<String>,
    },
    /// Führt einen einfachen Netzwerkscan durch
    Scan {
        /// Das Zielnetzwerk oder der Host
        target: String,
    },
    Test {
        /// Teste alle verfügbaren Commands
        #[arg(long)]
        all: bool,
    },
}


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ip => {
            commands::ip::handle_ip_command();
        }
        Commands::Ping { target } => {
            commands::ping::handle_ping_command(&target);
        }
        Commands::Logs { zip, analyze, query, lines, model, file } => {
            commands::logs::handle_logs_command(zip, analyze, query, lines, model, file).await;
        }
        Commands::Scan { target } => {
            commands::scan::handle_scan_command(&target);
        }
        Commands::Test { all } => {
            commands::test::handle_test_command(all);
        }
    }
}