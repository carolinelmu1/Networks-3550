use local_ip_address::local_ip;

fn main() {
    match local_ip() {
        Ok(ip) => println!("Local IP address: {}", ip),
        Err(e) => eprintln!("Failed to get local IP address: {}", e),
    }
}
