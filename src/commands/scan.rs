use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use std::str::FromStr;

pub fn handle_scan_command(target: &str) {
    println!("🔍 Starte Netzwerk-Scan für: {}", target);

    // Versuche die Ziel-IP zu parsen
    let target_ip = match parse_target(target) {
        Ok(ip) => ip,
        Err(e) => {
            println!("❌ Fehler beim Parsen des Ziels '{}': {}", target, e);
            return;
        }
    };

    println!("📡 Scanne IP-Adresse: {}", target_ip);
    println!("⏳ Führe Port-Scan durch...\n");

    scan_common_ports(target_ip);
}

fn parse_target(target: &str) -> Result<IpAddr, String> {
    // Versuche direkt als IP zu parsen
    if let Ok(ip) = IpAddr::from_str(target) {
        return Ok(ip);
    }

    // Wenn es eine IP-Range ist (z.B. 192.168.1.0/24), nehme die erste IP
    if target.contains('/') {
        let parts: Vec<&str> = target.split('/').collect();
        if parts.len() == 2 {
            if let Ok(ip) = IpAddr::from_str(parts[0]) {
                return Ok(ip);
            }
        }
    }

    // Fallback: Versuche als Hostname zu behandeln (vereinfacht)
    Err(format!("Konnte '{}' nicht als IP-Adresse parsen", target))
}

fn scan_common_ports(target_ip: IpAddr) {
    let common_ports = vec![
        21, 22, 23, 25, 53, 80, 110, 143, 443, 993, 995,
        135, 139, 445, 1433, 3389, 5432, 5900, 8080, 8443
    ];

    let mut open_ports = Vec::new();
    let timeout = Duration::from_millis(1000);

    for &port in &common_ports {
        let socket_addr = SocketAddr::new(target_ip, port);

        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(_) => {
                let service = get_service_name(port);
                println!("✅ Port {}: OFFEN ({})", port, service);
                open_ports.push(port);
            }
            Err(_) => {
                // Port ist geschlossen oder gefiltert - normalerweise nicht ausgeben
                // um die Ausgabe sauber zu halten
            }
        }
    }

    println!("\n📊 Scan-Zusammenfassung:");
    println!("🎯 Ziel: {}", target_ip);
    println!("🔍 Gescannte Ports: {}", common_ports.len());
    println!("✅ Offene Ports: {}", open_ports.len());

    if open_ports.is_empty() {
        println!("🚫 Keine offenen Ports in der Standard-Port-Liste gefunden.");
    } else {
        println!("📋 Offene Ports: {:?}", open_ports);

        println!("\n🔒 Sicherheitshinweise:");
        for &port in &open_ports {
            if let Some(warning) = get_security_warning(port) {
                println!("  ⚠️  Port {}: {}", port, warning);
            }
        }
    }
}

fn get_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        143 => "IMAP",
        443 => "HTTPS",
        993 => "IMAPS",
        995 => "POP3S",
        135 => "RPC",
        139 => "NetBIOS",
        445 => "SMB",
        1433 => "SQL Server",
        3389 => "RDP",
        5432 => "PostgreSQL",
        5900 => "VNC",
        8080 => "HTTP-Alt",
        8443 => "HTTPS-Alt",
        _ => "Unbekannt"
    }
}

fn get_security_warning(port: u16) -> Option<&'static str> {
    match port {
        21 => Some("FTP überträgt Daten unverschlüsselt"),
        23 => Some("Telnet ist unsicher - verwende SSH stattdessen"),
        25 => Some("SMTP sollte authentifiziert und verschlüsselt sein"),
        135 => Some("RPC kann Sicherheitsrisiken bergen"),
        139 => Some("NetBIOS sollte nicht öffentlich zugänglich sein"),
        445 => Some("SMB sollte nur in vertrauenswürdigen Netzwerken verwendet werden"),
        3389 => Some("RDP sollte mit starker Authentifizierung gesichert sein"),
        5900 => Some("VNC sollte mit Passwort und Verschlüsselung gesichert sein"),
        _ => None
    }
}

//fn is_well_known_port(port: u16) -> bool {
  //  match port {
    //    1..=1023 => true,
       // _ => false,
    //}
//}