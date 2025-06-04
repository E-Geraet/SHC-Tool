// src/main.rs
mod commands; // Damit main.rs die Module in commands/ kennt
// ... der Rest deines Codes

// src/main.rs
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Zeigt lokale IP-Adressen und Gateway Informationen an
    Ip,
    /// Sendet einen Ping an ein Ziel
    Ping {
        /// Die Ziel-IP-Adresse oder der Hostname
        target: String,
    },
    /// Sammelt und zippt Log-Dateien
    Logs {
        #[clap(long)] // Flag für --zip
        zip: bool,
        // Weitere Optionen für Logs später...
    },
    /// Führt einen einfachen Netzwerkscan durch
    Scan {
        /// Das Zielnetzwerk oder der Host
        target: String,
    },
    // Copyclip später hinzufügen, falls gewünscht
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ip => {
            println!("IP Befehl wird aufgerufen...");
            // Hier rufst du deine IP-Funktion auf
            // z.B. support_helper_cli::commands::ip::handle_ip_command();
        }
        Commands::Ping { target } => {
            println!("Ping Befehl wird aufgerufen für Ziel: {}", target);
            // z.B. support_helper_cli::commands::ping::handle_ping_command(&target);
        }
        Commands::Logs { zip } => {
            println!("Logs Befehl wird aufgerufen. Zippen: {}", zip);
            // z.B. support_helper_cli::commands::logs::handle_logs_command(zip);
        }
        Commands::Scan { target } => {
            println!("Scan Befehl wird aufgerufen für Ziel: {}", target);
            // z.B. support_helper_cli::commands::scan::handle_scan_command(&target);
        }
    }
}