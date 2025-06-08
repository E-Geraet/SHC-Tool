// src/main.rs
mod commands;
mod ollama;

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
    /// Sammelt und analysiert Log-Dateien
    Logs {
        /// Erstelle ZIP-Archiv der Log-Dateien
        #[clap(long)]
        zip: bool,
        /// Analysiere Logs mit KI (Ollama)
        #[clap(long)]
        analyze: bool,
        /// Frage für die KI-Analyse
        #[clap(long)]
        query: Option<String>,
        /// Anzahl der zu analysierenden Zeilen (Standard: 200)
        #[clap(long, default_value = "200")]
        lines: usize,
        /// Ollama-Modell (Standard: gemma2:2b)
        #[clap(long)]
        model: Option<String>,
    },
    /// Führt einen einfachen Netzwerkscan durch
    Scan {
        /// Das Zielnetzwerk oder der Host
        target: String,
    },
    /// Ollama-spezifische Befehle
    Ollama {
        #[clap(subcommand)]
        action: OllamaCommands,
    },
}

#[derive(Parser, Debug)]
enum OllamaCommands {
    /// Prüfe Ollama-Verfügbarkeit
    Status,
    /// Liste verfügbare Modelle
    Models,
    /// Interaktiver Chat-Modus für Log-Analyse
    Chat {
        /// Log-Datei zum Analysieren
        #[clap(long)]
        file: Option<String>,
        /// Anzahl der Zeilen (Standard: 200)
        #[clap(long, default_value = "200")]
        lines: usize,
        /// Ollama-Modell (Standard: gemma2:2b)
        #[clap(long)]
        model: Option<String>,
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
        Commands::Logs { zip, analyze, query, lines, model } => {
            commands::logs::handle_logs_command(zip, analyze, query, lines, model).await;
        }
        Commands::Scan { target } => {
            commands::scan::handle_scan_command(&target);
        }
        Commands::Ollama { action } => {
            handle_ollama_commands(action).await;
        }
    }
}

async fn handle_ollama_commands(action: OllamaCommands) {
    use crate::ollama::OllamaClient;

    match action {
        OllamaCommands::Status => {
            let ollama = OllamaClient::new(None, None);

            println!("🔍 Prüfe Ollama-Status...");

            if ollama.check_ollama_availability().await {
                println!("✅ Ollama ist verfügbar und läuft");

                match ollama.list_available_models().await {
                    Ok(models) => {
                        if !models.is_empty() {
                            println!("📋 Verfügbare Modelle: {}", models.len());
                            for model in models.iter().take(5) {
                                println!("  - {}", model);
                            }
                            if models.len() > 5 {
                                println!("  ... und {} weitere", models.len() - 5);
                            }
                        } else {
                            println!("⚠️  Keine Modelle installiert");
                            println!("💡 Installiere ein Modell: ollama pull gemma2:2b");
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Konnte Modelle nicht abrufen: {}", e);
                    }
                }
            } else {
                println!("❌ Ollama ist nicht verfügbar");
                println!("💡 Tipps:");
                println!("   - Installiere Ollama: https://ollama.ai");
                println!("   - Starte Ollama: ollama serve");
                println!("   - Installiere ein Modell: ollama pull gemma2:2b");
            }
        }

        OllamaCommands::Models => {
            let ollama = OllamaClient::new(None, None);

            println!("📋 Lade verfügbare Modelle...");

            match ollama.list_available_models().await {
                Ok(models) => {
                    if models.is_empty() {
                        println!("❌ Keine Modelle installiert");
                        println!("💡 Installiere ein Modell: ollama pull gemma2:2b");
                    } else {
                        println!("✅ Verfügbare Modelle ({}):", models.len());
                        for model in models {
                            println!("  📦 {}", model);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Fehler beim Abrufen der Modelle: {}", e);
                }
            }
        }

        OllamaCommands::Chat { file, lines, model } => {
            println!("💬 Interaktiver Chat-Modus wird gestartet...");
            println!("💡 Dieser Modus ist noch nicht implementiert");
            println!("   Geplante Features:");
            println!("   - Interaktive Fragen zu Log-Dateien");
            println!("   - Mehrere Fragen hintereinander");
            println!("   - Chat-History");

            if let Some(log_file) = file {
                println!("   Target-Datei: {}", log_file);
                println!("   Zeilen: {}", lines);
                if let Some(model_name) = model {
                    println!("   Modell: {}", model_name);
                }
            }
        }
    }
}