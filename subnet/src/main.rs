use std::env::args;

fn main() {
    // Iterate through args with index
    for (i, arg) in args().enumerate() {
        // subnet_args[0] is the name of the executable
        // subnet_args[1] the target ip address + cidr notation
        if i == 1 {
            let (network_id, broadcast_ip, first_host, last_host, next_network, total_num_ips, num_usable_hosts, subnet_mask) = calculate_subnet(arg);
            println!("IPv4 Subnet Critical Values:");
            println!("Network id: {}", network_id);
            println!("Broadcast ip: {}", broadcast_ip);
            println!("First host: {}", first_host);
            println!("Last host: {}", last_host);
            println!("Next network: {}", next_network);
            println!("Number of ip addresses: {} ({} usable)", total_num_ips, num_usable_hosts);
            println!("CIDR/Subnet Mask: {}", subnet_mask);
        }
    }
}

fn calculate_subnet(target_subnet: String) -> (String, String, String, String, String, u32, u32, &'static str) {
    // how to split strings into named variables
    let [target_ip, cidr]: [&str; 2] = target_subnet
        .split("/")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();
        
    println!("Target IP: {}", target_ip);
    println!("CIDR: {}", cidr);

    let my_cidr: u32 = cidr.parse().unwrap();
    let total_num_ips = 2u32.pow(32 - my_cidr);
    let num_usable_hosts = total_num_ips - 2;

    let subnet_mask = generate_subnet_mask(my_cidr);
    
    // Parse target_ip into u32
    let target_ip_val = parse_ipv4_address(target_ip);

    // Calculate network address value
    let network_address_value = target_ip_val & subnet_mask;

    // Calculate broadcast address value
    let broadcast_address_value = network_address_value | !subnet_mask;

    let network_id = format_ip_address(network_address_value);
    let broadcast_ip = format_ip_address(broadcast_address_value);

    let first_host_value = network_address_value + 1;
    let first_host = format_ip_address(first_host_value);

    let last_host_value = broadcast_address_value - 1;
    let last_host = format_ip_address(last_host_value);

    let next_network_value = broadcast_address_value + 1;
    let next_network = format_ip_address(next_network_value);

    (network_id, broadcast_ip, first_host, last_host, next_network, total_num_ips, num_usable_hosts, "255.255.0.0")
}

fn parse_ipv4_address(ip_addr: &str) -> u32 {
    let octets: Vec<u32> = ip_addr
        .split('.')
        .map(|octet| octet.parse().unwrap())
        .collect();

    // Combine octets into u32 representing the IP address
    (octets[0] << 24) | (octets[1] << 16) | (octets[2] << 8) | octets[3]
}

fn format_ip_address(ip_value: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        (ip_value >> 24) & 0xFF,
        (ip_value >> 16) & 0xFF,
        (ip_value >> 8) & 0xFF,
        ip_value & 0xFF
    )
}

fn generate_subnet_mask(cidr: u32) -> u32 {
    !0 << (32 - cidr)
}

// // split octets by "."
    // let [o1, o2, o3, o4]: [&str; 4] = target_ip
    //     .split(".")
    //     .collect::<Vec<&str>>()
    //     .try_into()
    //     .unwrap_or_default();

    // println!("1st Octet: {o1}");
    // println!("2nd Octet: {o2}");
    // println!("3rd Octet: {o3}");
    // println!("4th Octet: {o4}");

    // // Parse octet strings into mathable ints
    // let octet_strings: [&str; 4] = target_ip
    //     .split(".")
    //     .collect::<Vec<&str>>()
    //     .try_into()
    //     .unwrap_or_default();
        
    // for octet in octet_strings {
    //     let my_octet: i32 = octet.parse().unwrap();
    //     let octet_math = my_octet + 1;
    //     println!("{octet_math}");
    // }