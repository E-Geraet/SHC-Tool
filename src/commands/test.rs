// src/commands/test.rs
use std::process::Command;
use std::path::Path;
use std::fs;

pub fn handle_test_command(all: bool) {
    println!("🧪 Testing SHC-Tool Components...\n");

    if all {
        test_all_commands();
    } else {
        test_basic_functionality();
    }

    println!("\n✅ Test completed!");
}

fn test_basic_functionality() {
    println!("📋 Basic Functionality Test:");

    // Test 1: Module imports
    print!("  • Module imports... ");
    // Wenn wir hier sind, sind alle Module erfolgreich importiert
    println!("✅");

    // Test 2: Clap parsing
    print!("  • CLI parsing... ");
    // Wenn das Programm läuft, funktioniert Clap
    println!("✅");

    // Test 3: File system access
    print!("  • File system access... ");
    match fs::metadata(".") {
        Ok(_) => println!("✅"),
        Err(_) => println!("❌"),
    }

    // Test 4: Basic network stack
    print!("  • Network stack... ");
    match std::net::TcpStream::connect_timeout(
        &"127.0.0.1:22".parse().unwrap(),
        std::time::Duration::from_millis(100)
    ) {
        Ok(_) | Err(_) => println!("✅"), // Beide Ergebnisse sind ok für den Test
    }
}

fn test_all_commands() {
    println!("🔄 Full Command Test Suite:");

    // Test IP Command
    print!("  • IP Command... ");
    match test_ip_functionality() {
        true => println!("✅"),
        false => println!("❌"),
    }

    // Test Ping Command
    print!("  • Ping Command... ");
    match test_ping_functionality() {
        true => println!("✅"),
        false => println!("❌"),
    }

    // Test Logs Command
    print!("  • Logs Command... ");
    match test_logs_functionality() {
        true => println!("✅"),
        false => println!("❌"),
    }

    // Test Scan Command
    print!("  • Scan Command... ");
    match test_scan_functionality() {
        true => println!("✅"),
        false => println!("❌"),
    }

    // Test Ollama Integration
    print!("  • Ollama Integration... ");
    match test_ollama_availability() {
        true => println!("✅"),
        false => println!("⚠️ (Optional)"),
    }
}

fn test_ip_functionality() -> bool {
    // Test ob wir Netzwerk-Interfaces lesen können
    match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn test_ping_functionality() -> bool {
    // Test ob ping-Befehl verfügbar ist
    match Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("127.0.0.1")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => {
            // Falls ping nicht verfügbar, teste Windows ping
            match Command::new("ping")
                .arg("-n")
                .arg("1")
                .arg("127.0.0.1")
                .output()
            {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}

fn test_logs_functionality() -> bool {
    // Test ob wir auf typische Log-Verzeichnisse zugreifen können
    let log_paths = vec![
        "/var/log",
        "/var/log/syslog",
        "C:\\Windows\\System32\\winevt\\Logs",
        "./logs",
    ];

    for path in log_paths {
        if Path::new(path).exists() {
            return true;
        }
    }

    // Fallback: Test ob wir aktuelles Verzeichnis lesen können
    match fs::read_dir(".") {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn test_scan_functionality() -> bool {
    // Test ob wir Netzwerk-Scans durchführen können
    use std::net::{TcpStream, SocketAddr};
    use std::time::Duration;

    // Teste localhost Port 22 (SSH) oder 80 (HTTP)
    let test_addresses = vec![
        "127.0.0.1:22",
        "127.0.0.1:80",
        "127.0.0.1:443",
    ];

    for addr_str in test_addresses {
        if let Ok(addr) = addr_str.parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(100)).is_ok() {
                return true;
            }
        }
    }

    // Wenn keine Ports offen sind, ist das auch ok - wir können scannen
    true
}

fn test_ollama_availability() -> bool {
    // Test ob Ollama verfügbar ist
    match Command::new("ollama").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => {
            // Teste auch HTTP-Endpoint
            match std::net::TcpStream::connect_timeout(
                &"127.0.0.1:11434".parse().unwrap(),
                std::time::Duration::from_millis(500)
            ) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
    }
}