use std::net::{Ipv4Addr, Ipv6Addr};

use ipcalc::{Ipv4Netmask, Ipv4Network};


fn main() {
    let addr1: Ipv4Addr = [192, 168, 0, 1].into();

    let netmask: Ipv4Netmask = 24u8.try_into().unwrap();

    let network = Ipv4Network::new(addr1, netmask);

    for asd in network {
        let asd = asd.octets();
        println!("{0}.{1}.{2}.{3}\t{0:0>8b}.{1:0>8b}.{2:0>8b}.{3:0>8b}", asd[0], asd[1], asd[2], asd[3]);    
    }
    
    let asd = Ipv6Addr::LOCALHOST;

}
