use std::fs;
use std::path::Path;
use std::io::{self, BufRead, BufReader};
use crate::ollama::OllamaClient;

pub async fn handle_logs_command(zip: bool, analyze: bool, query: Option<String>, lines: usize, model: Option<String>) {
    println!("📋 Sammle System-Log-Informationen...");

    if zip {
        println!("📦 ZIP-Funktionalität wird vorbereitet...");
    }

    if analyze {
        if let Some(query_text) = query {
            println!("🔍 Starte Log-Analyse mit KI...");
            analyze_logs_with_ai(&query_text, lines, model).await;
        } else {
            println!("❌ Für die Analyse muss eine Frage mit --query angegeben werden.");
            return;
        }
    } else {
        // Normale Log-Übersicht
        show_log_overview();
    }

    if zip {
        println!("\n📦 Hinweis: ZIP-Funktionalität ist noch nicht implementiert.");
        println!("   Geplante Features:");
        println!("   - Sammlung relevanter Log-Dateien");
        println!("   - Komprimierung zu ZIP-Archiv");
        println!("   - Zeitstempel-basierte Dateinamen");
    }
}

async fn analyze_logs_with_ai(query: &str, lines: usize, model: Option<String>) {
    let ollama = OllamaClient::new(None, model);

    // Prüfe Ollama-Verfügbarkeit
    if !ollama.check_ollama_availability().await {
        println!("❌ Ollama ist nicht verfügbar!");
        println!("💡 Tipps:");
        println!("   - Stelle sicher, dass Ollama läuft: ollama serve");
        println!("   - Installiere ein Modell: ollama pull gemma2:2b");
        return;
    }

    println!("✅ Ollama ist verfügbar");

    // Finde und analysiere Log-Dateien
    let log_files = find_readable_log_files();

    if log_files.is_empty() {
        println!("❌ Keine lesbaren Log-Dateien gefunden.");
        return;
    }

    println!("🔍 Gefundene Log-Dateien: {}", log_files.len());

    for log_file in &log_files {
        println!("\n📄 Analysiere: {}", log_file);

        match read_last_lines(log_file, lines) {
            Ok(content) => {
                if content.trim().is_empty() {
                    println!("⚠️  Datei ist leer oder enthält nur Leerzeichen");
                    continue;
                }

                match ollama.analyze_log(&content, query, lines).await {
                    Ok(analysis) => {
                        println!("✅ Analyse-Ergebnis:");
                        println!("{}", analysis);
                        println!("{}", "=".repeat(50));
                    }
                    Err(e) => {
                        println!("❌ Fehler bei der Analyse: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Konnte Datei nicht lesen: {}", e);
            }
        }
    }
}

fn show_log_overview() {
    // Liste häufiger Log-Verzeichnisse unter Linux
    let log_paths = vec![
        "/var/log/syslog",
        "/var/log/auth.log",
        "/var/log/kern.log",
        "/var/log/dmesg",
        "/var/log/messages",
        "/var/log/apache2/",
        "/var/log/nginx/",
        "/var/log/mysql/",
    ];

    println!("\n🔍 Überprüfe Log-Dateien und -Verzeichnisse:");

    for log_path in &log_paths {
        check_log_path(log_path);
    }

    // Systemd journal logs (falls verfügbar)
    println!("\n📖 Systemd Journal Status:");
    check_systemd_journal();
}

fn find_readable_log_files() -> Vec<String> {
    let mut log_files = Vec::new();

    let potential_paths = vec![
        "/var/log/syslog",
        "/var/log/auth.log",
        "/var/log/kern.log",
        "/var/log/messages",
        "/var/log/dmesg",
    ];

    for path in potential_paths {
        if Path::new(path).exists() && can_read_file(path) {
            log_files.push(path.to_string());
        }
    }

    log_files
}

fn can_read_file(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn read_last_lines(file_path: &str, num_lines: usize) -> Result<String, std::io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let lines = lines?;

    let start_index = if lines.len() > num_lines {
        lines.len() - num_lines
    } else {
        0
    };

    Ok(lines[start_index..].join("\n"))
}

fn check_log_path(path: &str) {
    let path_obj = Path::new(path);

    if path_obj.exists() {
        if path_obj.is_file() {
            match fs::metadata(path) {
                Ok(metadata) => {
                    let size = metadata.len();
                    let readable = if can_read_file(path) { "✅" } else { "🔒" };
                    println!("  {} {} (Datei, {} Bytes)", readable, path, size);
                }
                Err(_) => {
                    println!("  ❓ {} (Datei, Größe unbekannt)", path);
                }
            }
        } else if path_obj.is_dir() {
            match fs::read_dir(path) {
                Ok(entries) => {
                    let count = entries.count();
                    println!("  ✅ {} (Verzeichnis, {} Einträge)", path, count);
                }
                Err(_) => {
                    println!("  ❓ {} (Verzeichnis, Anzahl unbekannt)", path);
                }
            }
        }
    } else {
        println!("  ❌ {} (nicht vorhanden)", path);
    }
}

fn check_systemd_journal() {
    use std::process::Command;

    // Versuche systemctl status zu verwenden um zu prüfen ob systemd verfügbar ist
    let output = Command::new("systemctl")
        .arg("--version")
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("  ✅ Systemd ist verfügbar");
                println!("  💡 Verwende 'journalctl' für detaillierte System-Logs");

                // Zeige die letzten paar Zeilen des Journals
                let journal_output = Command::new("journalctl")
                    .arg("-n")
                    .arg("5")
                    .arg("--no-pager")
                    .output();

                match journal_output {
                    Ok(journal_result) => {
                        if journal_result.status.success() {
                            println!("  📋 Letzte Journal-Einträge:");
                            if let Ok(journal_str) = String::from_utf8(journal_result.stdout) {
                                for line in journal_str.lines().take(3) {
                                    println!("    {}", line);
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("  ⚠️  Konnte Journal nicht lesen (Berechtigung erforderlich?)");
                    }
                }
            } else {
                println!("  ❌ Systemd nicht verfügbar oder nicht erreichbar");
            }
        }
        Err(_) => {
            println!("  ❌ Systemd nicht gefunden");
        }
    }
}