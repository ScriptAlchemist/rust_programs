use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    // Set the timeout for the TcpStreams to 1 second
    let timeout = Duration::new(1, 0);

    // Iterate over all IP addresses in the local network
    for ip in 0..=u32::MAX {
        let ipv4_addr = Ipv4Addr::from(ip);

        // Iterate over all ports
        for port in 1..=65535 {
            // Construct a SocketAddr from the current IP address and port
            let addr = SocketAddr::new(IpAddr::V4(ipv4_addr), port);

            // Try to create a TcpStream to the current SocketAddr
            match TcpStream::connect_timeout(&addr, timeout) {
                // If the stream was successfully created, the port is open
                Ok(_) => println!("Port {} on {} is open", port, ipv4_addr),
                // If an error occurred, the port is closed
                Err(_) => continue,
            }
        }
    }
}

