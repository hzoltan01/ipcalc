use std::{fmt::{self, Debug, Display}, net::Ipv4Addr};

use crate::Ipv4Netmask;

mod iterator;



pub enum Network {
    V4(Ipv4Network),
    V6(Ipv6Network),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Network {
    network_addr: Ipv4Addr,
    netmask: Ipv4Netmask,
}

impl Ipv4Network {

    pub const fn netmask(&self) -> Ipv4Netmask {
        self.netmask
    }

    pub const fn new(address: Ipv4Addr, netmask: Ipv4Netmask) -> Self {

        let addr_bits = address.to_bits();
        let mask_bits = netmask.to_bits();

        let network_bits = addr_bits & mask_bits;
        let network_addr = Ipv4Addr::from_bits(network_bits);

        Self { network_addr, netmask }
    }

    pub fn from_hosts(first: Ipv4Addr, last: Ipv4Addr) -> Self {
        let xor_res = first.to_bits() ^ last.to_bits();
        let cidr = u8::try_from(xor_res.leading_zeros()).unwrap();

        let mask = Ipv4Netmask::from_cidr(cidr).unwrap();
        Self::new(first, mask)
    }

    pub const fn network_addr(&self) -> Ipv4Addr {
        self.network_addr
        
    }

    pub const fn broadcast_addr(&self) -> Ipv4Addr {
        let addr_bits = self.network_addr.to_bits();
        let mask_bits = !self.netmask.to_bits();

        let network_bits = addr_bits | mask_bits;
        Ipv4Addr::from_bits(network_bits)
    }

    pub fn min_host(&self) -> Option<Ipv4Addr> {
        match self.netmask.cidr() {
            32 => None,
            31 => Some(self.network_addr()),
            _ => Some(Ipv4Addr::from_bits(self.network_addr().to_bits() + 1)),
        }
    }

    pub const fn max_host(&self) -> Option<Ipv4Addr> {
        match self.netmask.cidr() {
            32 => None,
            31 => Some(self.broadcast_addr()),
            _ => Some(Ipv4Addr::from_bits(self.broadcast_addr().to_bits() - 1)),
        }
    }

    pub const fn contains(&self, addr: Ipv4Addr) -> bool {
        let shift = 32 - self.netmask.cidr();
        let network_bits = self.network_addr.to_bits() >> shift;
        let a_network_bits = addr.to_bits() >> shift;

        network_bits == a_network_bits
    }

}

impl From<(Ipv4Addr, Ipv4Netmask)> for Ipv4Network {
    fn from(value: (Ipv4Addr, Ipv4Netmask)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(Ipv4Addr, Ipv4Addr)> for Ipv4Network {
    fn from(value: (Ipv4Addr, Ipv4Addr)) -> Self {
        Self::from_hosts(value.0, value.1)
    }
}

impl Display for Ipv4Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{}{}", self.network_addr(), self.netmask))
    }
}

impl Debug for Ipv4Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl IntoIterator for Ipv4Network {
    type Item = Ipv4Addr;

    type IntoIter = iterator::NetworkIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.network_addr(), self.broadcast_addr())
    }
}
pub struct Ipv6Network;
