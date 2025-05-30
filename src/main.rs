use std::net::IpAddr;
use clap::Parser;
use std::process::Command;






fn main() {
    match local_ip_address::local_ip() { // Zeile für den IP-Check
        Ok(ip_addr) => {
            println!("Meine lokale IP-Adresse ist: {}", ip_addr);
        }
        Err(e) => {
            // Fehlermeldung
            eprintln!("Konnte lokale IP-Adresse nicht ermitteln: {}", e);
        }
    }

}

