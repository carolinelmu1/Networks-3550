
use std::io;
use std::convert::TryInto;

fn main() {
    println!("Please enter an IPv6 address (e.g., 2001:0db8:85a3:0000:0000:8a2e:0370:7334):");

    let mut ipv6_address = String::new();
    io::stdin()
        .read_line(&mut ipv6_address)
        .expect("Failed to read line");

    // Trim whitespace and newline characters from the input
    let ipv6_address = ipv6_address.trim();

    let hextets: Vec<&str> = ipv6_address.split(":").collect();
    if hextets.len() != 8 {
        panic!("Invalid IPv6 address format: expected 8 hextets");
    }

    let hextets: [&str; 8] = ipv6_address
        .split(":")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    for hextet in hextets {
        println!("{}", hextet);
    }
}