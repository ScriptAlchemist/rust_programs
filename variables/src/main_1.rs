use std::fmt;

#[allow(dead_code)]

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{0}.{1}.{2}.{3}", a,b,c,d),
            IpAddr::V6(addr) => write!(f, "{}", addr),
        }
    }
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);


    println!("The home IP address is ({})", home);
}

