use std::fs;
use std::path::Path;

pub fn handle_logs_command(zip: bool) {
    println!("📋 Sammle System-Log-Informationen...");

    if zip {
        println!("📦 ZIP-Funktionalität wird vorbereitet...");
    }

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

    if zip {
        println!("\n📦 Hinweis: ZIP-Funktionalität ist noch nicht implementiert.");
        println!("   Geplante Features:");
        println!("   - Sammlung relevanter Log-Dateien");
        println!("   - Komprimierung zu ZIP-Archiv");
        println!("   - Zeitstempel-basierte Dateinamen");
    }
}

fn check_log_path(path: &str) {
    let path_obj = Path::new(path);

    if path_obj.exists() {
        if path_obj.is_file() {
            match fs::metadata(path) {
                Ok(metadata) => {
                    let size = metadata.len();
                    println!("  ✅ {} (Datei, {} Bytes)", path, size);
                }
                Err(_) => {
                    println!("  ✅ {} (Datei, Größe unbekannt)", path);
                }
            }
        } else if path_obj.is_dir() {
            match fs::read_dir(path) {
                Ok(entries) => {
                    let count = entries.count();
                    println!("  ✅ {} (Verzeichnis, {} Einträge)", path, count);
                }
                Err(_) => {
                    println!("  ✅ {} (Verzeichnis, Anzahl unbekannt)", path);
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