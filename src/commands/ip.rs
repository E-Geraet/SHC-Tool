use netdev;

pub fn handle_ip_command() {
    println!("=== Netzwerk Interface Informationen ===\n");

    // Zuerst das Standard-Interface anzeigen
    show_default_interface();

    println!("\n{}\n", "=".repeat(50));

    // Dann alle Interfaces anzeigen
    show_all_interfaces();
}

fn show_default_interface() {
    println!("🔹 Standard Network Interface:");
    match netdev::get_default_interface() {
        Ok(interface) => {
            print_interface_details(&interface, true);
        }
        Err(e) => {
            println!("❌ Fehler beim Abrufen des Standard-Interfaces: {}", e);
        }
    }
}

fn show_all_interfaces() {
    println!("🔹 Alle Network Interfaces:");
    let interfaces = netdev::get_interfaces();

    if interfaces.is_empty() {
        println!("❌ Keine Netzwerk-Interfaces gefunden.");
        return;
    }

    for (index, interface) in interfaces.iter().enumerate() {
        println!("\n--- Interface {} ---", index + 1);
        print_interface_details(interface, false);
    }
}

fn print_interface_details(interface: &netdev::Interface, show_gateway: bool) {
    println!("  Name: {}", interface.name);

    if let Some(friendly_name) = &interface.friendly_name {
        println!("  Anzeigename: {}", friendly_name);
    }

    if let Some(description) = &interface.description {
        println!("  Beschreibung: {}", description);
    }

    println!("  Index: {}", interface.index);
    println!("  Typ: {}", interface.if_type.name());

    // Status-Flags
    let mut status_flags = Vec::new();
    if interface.is_up() { status_flags.push("UP"); }
    if interface.is_running() { status_flags.push("RUNNING"); }
    if interface.is_loopback() { status_flags.push("LOOPBACK"); }
    if interface.is_physical() { status_flags.push("PHYSICAL"); }
    if interface.is_multicast() { status_flags.push("MULTICAST"); }
    if interface.is_broadcast() { status_flags.push("BROADCAST"); }
    if interface.is_point_to_point() { status_flags.push("P2P"); }
    if interface.is_tun() { status_flags.push("TUN"); }

    if !status_flags.is_empty() {
        println!("  Status: {}", status_flags.join(", "));
    }

    // MAC-Adresse
    if let Some(mac_addr) = interface.mac_addr {
        println!("  MAC-Adresse: {}", mac_addr);
    } else {
        println!("  MAC-Adresse: Nicht verfügbar");
    }

    // IPv4-Adressen
    if !interface.ipv4.is_empty() {
        println!("  IPv4-Adressen:");
        for ipv4 in &interface.ipv4 {
            println!("    - {} (Netzmaske: {})", ipv4.addr(), ipv4.netmask());
        }

        // Globale IPv4-Adressen hervorheben
        if interface.has_global_ipv4() {
            let global_addrs = interface.global_ipv4_addrs();
            println!("  🌐 Globale IPv4-Adressen:");
            for ip in global_addrs {
                println!("    - {}", ip);
            }
        }
    } else {
        println!("  IPv4-Adressen: Keine");
    }

    // IPv6-Adressen
    if !interface.ipv6.is_empty() {
        println!("  IPv6-Adressen:");
        for (ipv6, scope_id) in interface.ipv6.iter().zip(&interface.ipv6_scope_ids) {
            println!("    - {} (Scope ID: {})", ipv6.addr(), scope_id);
        }

        // Globale IPv6-Adressen hervorheben
        if interface.has_global_ipv6() {
            let global_addrs = interface.global_ipv6_addrs();
            println!("  🌐 Globale IPv6-Adressen:");
            for ip in global_addrs {
                println!("    - {}", ip);
            }
        }
    } else {
        println!("  IPv6-Adressen: Keine");
    }

    // Gateway-Informationen (nur für Standard-Interface oder wenn explizit angefordert)
    if show_gateway {
        if let Some(gateway) = &interface.gateway {
            println!("  🚪 Standard-Gateway:");
            println!("    MAC-Adresse: {}", gateway.mac_addr);
            if !gateway.ipv4.is_empty() {
                println!("    IPv4: {:?}", gateway.ipv4);
            }
            if !gateway.ipv6.is_empty() {
                println!("    IPv6: {:?}", gateway.ipv6);
            }
        } else {
            println!("  🚪 Standard-Gateway: Nicht gefunden");
        }

        // DNS-Server
        if !interface.dns_servers.is_empty() {
            println!("  🌐 DNS-Server:");
            for dns in &interface.dns_servers {
                println!("    - {}", dns);
            }
        } else {
            println!("  🌐 DNS-Server: Keine konfiguriert");
        }
    }

    // Übertragungsgeschwindigkeiten
    if let Some(tx_speed) = interface.transmit_speed {
        println!("  📤 Sende-Geschwindigkeit: {} Mbps", tx_speed / 1_000_000);
    }
    if let Some(rx_speed) = interface.receive_speed {
        println!("  📥 Empfangs-Geschwindigkeit: {} Mbps", rx_speed / 1_000_000);
    }

    // MTU
    if let Some(mtu) = interface.mtu {
        println!("  📏 MTU: {} Bytes", mtu);
    }

    // Standard-Interface Kennzeichnung
    if interface.default {
        println!("  ⭐ Dies ist das Standard-Interface");
    }
}