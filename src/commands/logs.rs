use std::fs;
use std::path::Path;
use crate::ollama::OllamaClient;

pub async fn handle_logs_command(
    zip: bool,
    analyze: bool,
    query: Option<String>,
    lines: usize,
    model: Option<String>,
    file: Option<String>  // Neuer Parameter
) {
    println!("📋 Sammle System-Log-Informationen...");

    if zip {
        println!("📦 ZIP-Funktionalität wird vorbereitet...");
    }

    if analyze {
        if let Some(query_text) = query {
            println!("🔍 Starte Log-Analyse mit KI...");
            analyze_logs_with_ai(&query_text, lines, model, file).await;
        } else {
            println!("❌ Für die Analyse muss eine Frage mit --query angegeben werden.");
            return;
        }
    } else {
        // Normale Log-Übersicht
        if let Some(file_path) = file {
            show_specific_log(&file_path);
        } else {
            show_log_overview();
        }
    }

    if zip {
        println!("\n📦 Hinweis: ZIP-Funktionalität ist noch nicht implementiert.");
        println!("   Geplante Features:");
        println!("   - Sammlung relevanter Log-Dateien");
        println!("   - Komprimierung zu ZIP-Archiv");
        println!("   - Zeitstempel-basierte Dateinamen");
    }
}

async fn analyze_logs_with_ai(
    query: &str,
    lines: usize,
    model: Option<String>,
    file: Option<String>  // Neuer Parameter
) {
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

    // Bestimme welche Log-Dateien analysiert werden sollen
    let log_files = if let Some(specific_file) = file {
        // Verwende spezifische Datei
        if Path::new(&specific_file).exists() && can_read_file(&specific_file) {
            vec![specific_file]
        } else {
            println!("❌ Datei '{}' existiert nicht oder ist nicht lesbar.", specific_file);
            return;
        }
    } else {
        // Verwende automatische Suche
        find_readable_log_files()
    };

    if log_files.is_empty() {
        println!("❌ Keine lesbaren Log-Dateien gefunden.");
        return;
    }

    println!("🔍 Zu analysierende Log-Dateien: {}", log_files.len());

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

fn show_specific_log(file_path: &str) {
    println!("📄 Informationen zu: {}", file_path);

    let path_obj = Path::new(file_path);

    if !path_obj.exists() {
        println!("❌ Datei existiert nicht: {}", file_path);
        return;
    }

    match fs::metadata(file_path) {
        Ok(metadata) => {
            let size = metadata.len();
            let readable = if can_read_file(file_path) { "✅" } else { "🔒" };
            println!("  Status: {} Lesbar", readable);
            println!("  Größe: {} Bytes", size);

            if size > 0 {
                // Zeige die letzten paar Zeilen als Vorschau
                match read_last_lines(file_path, 5) {
                    Ok(content) => {
                        println!("  📋 Letzte 5 Zeilen:");
                        for line in content.lines() {
                            println!("    {}", line);
                        }
                    }
                    Err(e) => {
                        println!("  ⚠️  Konnte Vorschau nicht laden: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Fehler beim Lesen der Metadaten: {}", e);
        }
    }
}

fn show_log_overview() {
    println!("🔍 Verfügbare Log-Dateien:");

    let log_files = find_readable_log_files();

    if log_files.is_empty() {
        println!("❌ Keine lesbaren Log-Dateien gefunden.");
        return;
    }

    for log_file in &log_files {
        print!("  📄 {}", log_file);

        match fs::metadata(log_file) {
            Ok(metadata) => {
                let size = metadata.len();
                println!(" ({}KB)", size / 1024);
            }
            Err(_) => {
                println!(" (Größe unbekannt)");
            }
        }
    }

    println!("\n💡 Tipp: Verwende --analyze --query \"deine Frage\" für KI-Analyse");
    println!("💡 Tipp: Verwende --file /pfad/zur/datei für spezifische Dateien");
}

pub fn find_readable_log_files() -> Vec<String> {
    let potential_paths = vec![
        "/var/log/syslog",
        "/var/log/auth.log",
        "/var/log/kern.log",
        "/var/log/messages",
        "/var/log/dmesg",
    ];

    potential_paths
        .into_iter()
        .filter(|path| Path::new(path).exists() && can_read_file(path))
        .map(|s| s.to_string())
        .collect()
}

fn can_read_file(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn read_last_lines(file_path: &str, max_lines: usize) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    let start_index = if lines.len() > max_lines {
        lines.len() - max_lines
    } else {
        0
    };

    Ok(lines[start_index..].join("\n"))
}