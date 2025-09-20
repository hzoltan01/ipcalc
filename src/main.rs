use std::net::Ipv4Addr;

use ipcalc::{Ipv4Netmask, Ipv4Network};


fn main() {
    let network = Ipv4Network::new([192,168,3,33].into(), Ipv4Netmask::from_cidr(23).unwrap());
    println!("{}", network.network_addr());
    println!("{}", network.broadcast_addr());

}
