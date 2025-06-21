use netdev;

pub fn handle_ip_command() {
    println!("=== Network Interface Information ===\n");

    // Show default interface first
    show_default_interface();

    println!("\n{}\n", "=".repeat(50));

    // Then show all interfaces
    show_all_interfaces();
}

fn show_default_interface() {
    println!("🔹 Default Network Interface:");
    match netdev::get_default_interface() {
        Ok(interface) => {
            print_interface_details(&interface, true);
        }
        Err(e) => {
            println!("❌ Error getting default interface: {}", e);
        }
    }
}

fn show_all_interfaces() {
    println!("🔹 All Network Interfaces:");
    let interfaces = netdev::get_interfaces();

    if interfaces.is_empty() {
        println!("❌ No network interfaces found.");
        return;
    }

    for (i, interface) in interfaces.iter().enumerate() {
        println!("\n--- Interface {} ---", i + 1);
        print_interface_details(interface, false);
    }
}

fn print_interface_details(interface: &netdev::Interface, show_gateway: bool) {
    println!("  Name: {}", interface.name);

    if let Some(display_name) = &interface.friendly_name {
        println!("  Display Name: {}", display_name);
    }

    if let Some(desc) = &interface.description {
        println!("  Description: {}", desc);
    }

    println!("  Index: {}", interface.index);
    println!("  Type: {}", interface.if_type.name());

    // Build status flags - could be more elegant but this works
    let mut flags = Vec::new();
    if interface.is_up() { flags.push("UP"); }
    if interface.is_running() { flags.push("RUNNING"); }
    if interface.is_loopback() { flags.push("LOOPBACK"); }
    if interface.is_physical() { flags.push("PHYSICAL"); }
    if interface.is_multicast() { flags.push("MULTICAST"); }
    if interface.is_broadcast() { flags.push("BROADCAST"); }
    if interface.is_point_to_point() { flags.push("P2P"); }
    if interface.is_tun() { flags.push("TUN"); }

    if !flags.is_empty() {
        println!("  Status: {}", flags.join(", "));
    }

    // MAC address
    match interface.mac_addr {
        Some(mac) => println!("  MAC Address: {}", mac),
        None => println!("  MAC Address: Not available"),
    }

    // IPv4 addresses
    if !interface.ipv4.is_empty() {
        println!("  IPv4 Addresses:");
        for ipv4 in &interface.ipv4 {
            println!("    - {} (Netmask: {})", ipv4.addr(), ipv4.netmask());
        }

        // Highlight global IPv4 addresses
        if interface.has_global_ipv4() {
            let global_addrs = interface.global_ipv4_addrs();
            println!("  🌐 Global IPv4 Addresses:");
            for ip in global_addrs {
                println!("    - {}", ip);
            }
        }
    } else {
        println!("  IPv4 Addresses: None");
    }

    // IPv6 addresses
    if !interface.ipv6.is_empty() {
        println!("  IPv6 Addresses:");
        // TODO: This zip could be cleaner, but it works for now
        for (ipv6, scope_id) in interface.ipv6.iter().zip(&interface.ipv6_scope_ids) {
            println!("    - {} (Scope ID: {})", ipv6.addr(), scope_id);
        }

        if interface.has_global_ipv6() {
            let global_addrs = interface.global_ipv6_addrs();
            println!("  🌐 Global IPv6 Addresses:");
            for ip in global_addrs {
                println!("    - {}", ip);
            }
        }
    } else {
        println!("  IPv6 Addresses: None");
    }

    // Gateway info (only for default interface)
    if show_gateway {
        match &interface.gateway {
            Some(gateway) => {
                println!("  🚪 Default Gateway:");
                println!("    MAC Address: {}", gateway.mac_addr);
                if !gateway.ipv4.is_empty() {
                    println!("    IPv4: {:?}", gateway.ipv4);
                }
                if !gateway.ipv6.is_empty() {
                    println!("    IPv6: {:?}", gateway.ipv6);
                }
            }
            None => println!("  🚪 Default Gateway: Not found"),
        }

        // DNS servers
        if !interface.dns_servers.is_empty() {
            println!("  🌐 DNS Servers:");
            for dns in &interface.dns_servers {
                println!("    - {}", dns);
            }
        } else {
            println!("  🌐 DNS Servers: None configured");
        }
    }

    // Transmission speeds
    if let Some(tx_speed) = interface.transmit_speed {
        println!("  📤 Transmit Speed: {} Mbps", tx_speed / 1_000_000);
    }
    if let Some(rx_speed) = interface.receive_speed {
        println!("  📥 Receive Speed: {} Mbps", rx_speed / 1_000_000);
    }

    // MTU
    if let Some(mtu) = interface.mtu {
        println!("  📏 MTU: {} bytes", mtu);
    }

    // Default interface marker
    if interface.default {
        println!("  ⭐ This is the default interface");
    }
}