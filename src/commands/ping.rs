use std::process::Command;
use std::io::{self, Write};

pub fn handle_ping_command(target: &str) {
    println!("🏓 Pinge Ziel: {}", target);

    // Ping-Befehl für Linux/Unix ausführen
    let output = Command::new("ping")
        .arg("-c")
        .arg("4") // 4 Ping-Pakete senden
        .arg(target)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                // Erfolgreiche Ausgabe
                println!("✅ Ping erfolgreich:");
                match String::from_utf8(result.stdout.clone()) {
                    Ok(stdout_str) => {
                        println!("{}", stdout_str);
                    }
                    Err(_) => {
                        println!("📄 Ping-Ausgabe (Rohdaten):");
                        io::stdout().write_all(&result.stdout).unwrap_or(());
                    }
                }
            } else {
                // Ping fehlgeschlagen
                println!("❌ Ping fehlgeschlagen:");
                match String::from_utf8(result.stderr) {
                    Ok(stderr_str) => {
                        println!("{}", stderr_str);
                    }
                    Err(_) => {
                        println!("Fehler beim Dekodieren der Fehlerausgabe");
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Fehler beim Ausführen des Ping-Befehls: {}", e);
            println!("💡 Tipp: Stelle sicher, dass 'ping' auf diesem System verfügbar ist.");
        }
    }
}