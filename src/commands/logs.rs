use std::fs;
use std::path::Path;
use std::io::Write;
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::ollama::OllamaClient;

pub async fn handle_logs_command(
    zip: bool,
    analyze: bool,
    query: Option<String>,
    lines: usize,
    model: Option<String>,
    file: Option<String>,
) {
    println!("📋 === System Log Handler ===\n");

    if zip {
        create_log_archive().await;
        return;
    }

    if analyze {
        if let Some(query_text) = query {
            analyze_logs_with_ai(&query_text, lines, model, file).await;
        } else {
            println!("❌ Für die Analyse ist eine Frage erforderlich. Verwende --query \"Deine Frage\"");
        }
        return;
    }

    // Standard: Zeige verfügbare Log-Dateien
    show_available_logs();
}

fn show_available_logs() {
    println!("🔍 Verfügbare Log-Dateien:");

    let log_paths = get_common_log_paths();
    let mut found_logs = Vec::new();

    for path in log_paths {
        if Path::new(&path).exists() {
            match fs::metadata(&path) {
                Ok(metadata) => {
                    let size = metadata.len();
                    let size_str = format_file_size(size);
                    println!("  ✅ {} ({})", path, size_str);
                    found_logs.push(path);
                }
                Err(_) => {
                    println!("  ❌ {} (nicht lesbar)", path);
                }
            }
        }
    }

    if found_logs.is_empty() {
        println!("  ⚠️  Keine Standard-Log-Dateien gefunden.");
        println!("\n💡 Versuche:");
        println!("  • shc-tool logs --file /pfad/zu/deiner/logdatei");
        println!("  • shc-tool logs --zip (erstellt Archiv aller verfügbaren Logs)");
    } else {
        println!("\n📊 Zusammenfassung:");
        println!("  • {} Log-Dateien gefunden", found_logs.len());
        println!("\n💡 Nächste Schritte:");
        println!("  • shc-tool logs --zip (erstellt ZIP-Archiv)");
        println!("  • shc-tool logs --analyze --query \"Was ist das Problem?\"");
    }
}

fn get_common_log_paths() -> Vec<String> {
    let mut paths = Vec::new();

    // Linux/Unix Log-Pfade
    let unix_logs = vec![
        "/var/log/syslog",
        "/var/log/messages",
        "/var/log/kern.log",
        "/var/log/auth.log",
        "/var/log/daemon.log",
        "/var/log/mail.log",
        "/var/log/user.log",
        "/var/log/boot.log",
        "/var/log/dmesg",
        "/var/log/cron.log",
        "/var/log/apache2/error.log",
        "/var/log/apache2/access.log",
        "/var/log/nginx/error.log",
        "/var/log/nginx/access.log",
    ];

    // Windows Log-Pfade (als Referenz, WER-Dateien sind schwieriger zu lesen)
    let windows_logs = vec![
        "C:\\Windows\\System32\\winevt\\Logs\\System.evtx",
        "C:\\Windows\\System32\\winevt\\Logs\\Application.evtx",
        "C:\\Windows\\System32\\winevt\\Logs\\Security.evtx",
    ];

    paths.extend(unix_logs.iter().map(|s| s.to_string()));
    paths.extend(windows_logs.iter().map(|s| s.to_string()));

    paths
}

async fn create_log_archive() {
    println!("📦 Erstelle Log-Archiv...");

    let archive_name = format!("system_logs_{}.zip",
                               chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    match create_zip_archive(&archive_name).await {
        Ok(file_count) => {
            println!("✅ Archiv erstellt: {}", archive_name);
            println!("📁 {} Dateien archiviert", file_count);
        }
        Err(e) => {
            println!("❌ Fehler beim Erstellen des Archivs: {}", e);
        }
    }
}

async fn create_zip_archive(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let file = fs::File::create(filename)?;
    let mut zip = ZipWriter::new(file);
    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let log_paths = get_common_log_paths();
    let mut file_count = 0;

    for log_path in log_paths {
        if Path::new(&log_path).exists() {
            if let Ok(contents) = fs::read(&log_path) {
                let filename_in_zip = log_path.replace("/", "_").replace("\\", "_");
                zip.start_file(filename_in_zip, options)?;
                zip.write_all(&contents)?;
                file_count += 1;
                println!("  ✅ Hinzugefügt: {}", log_path);
            }
        }
    }

    zip.finish()?;
    Ok(file_count)
}

async fn analyze_logs_with_ai(
    query: &str,
    lines: usize,
    model: Option<String>,
    file_path: Option<String>,
) {
    println!("🤖 Starte Log-Analyse mit AI...");

    // Prüfe Ollama-Verfügbarkeit
    let ollama_client = OllamaClient::new(None, model);

    if !ollama_client.check_ollama_availability().await {
        println!("❌ Ollama ist nicht verfügbar. Stelle sicher, dass Ollama läuft:");
        println!("   curl -fsSL https://ollama.ai/install.sh | sh");
        println!("   ollama serve");
        return;
    }

    // Bestimme welche Log-Datei analysiert werden soll
    let log_content = if let Some(specific_file) = file_path {
        read_log_file(&specific_file, lines)
    } else {
        read_default_log_file(lines)
    };

    let log_content = match log_content {
        Ok(content) => content,
        Err(e) => {
            println!("❌ Fehler beim Lesen der Log-Datei: {}", e);
            return;
        }
    };

    if log_content.trim().is_empty() {
        println!("⚠️  Log-Datei ist leer oder konnte nicht gelesen werden.");
        return;
    }

    // Führe AI-Analyse durch
    match ollama_client.analyze_log(&log_content, query, lines).await {
        Ok(analysis) => {
            println!("\n📊 === AI-Analyse Ergebnis ===");
            println!("{}", analysis);
            println!("\n=== Ende der Analyse ===");
        }
        Err(e) => {
            println!("❌ Fehler bei der AI-Analyse: {}", e);
        }
    }
}

fn read_default_log_file(lines: usize) -> Result<String, Box<dyn std::error::Error>> {
    let default_paths = vec![
        "/var/log/syslog",
        "/var/log/messages",
        "/var/log/kern.log",
    ];

    for path in default_paths {
        if Path::new(path).exists() {
            println!("📖 Lese Log-Datei: {}", path);
            return read_log_file(path, lines);
        }
    }

    Err("Keine Standard-Log-Datei gefunden".into())
}

fn read_log_file(file_path: &str, lines: usize) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    let log_lines: Vec<&str> = content.lines().collect();
    let total_lines = log_lines.len();

    if total_lines == 0 {
        return Ok("Log-Datei ist leer".to_string());
    }

    // Nimm die letzten N Zeilen
    let start_index = if total_lines > lines {
        total_lines - lines
    } else {
        0
    };

    let selected_lines = &log_lines[start_index..];
    let result = selected_lines.join("\n");

    println!("📊 Gelesen: {} von {} Zeilen aus {}",
             selected_lines.len(), total_lines, file_path);

    Ok(result)
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}