// src/main.rs
mod commands; // Damit main.rs die Module in commands/ kennt

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
            // IP-Funktion aufrufen
            commands::ip::handle_ip_command();
        }
        Commands::Ping { target } => {
            commands::ping::handle_ping_command(&target);
        }
        Commands::Logs { zip } => {
            commands::logs::handle_logs_command(zip);
        }
        Commands::Scan { target } => {
            commands::scan::handle_scan_command(&target);
        }
    }
}