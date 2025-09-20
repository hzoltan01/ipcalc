use std::net::Ipv4Addr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Netmask {
    V4(Ipv4Netmask),
    V6(Ipv6Netmask),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Netmask {
    cidr: u8,
}

impl Ipv4Netmask {
    pub const fn cidr(&self) -> u8 {
        self.cidr
    }

    pub const fn as_cidr(&self) -> &u8 {
        &self.cidr
    }

    pub const fn to_bytes(&self) -> [u8; 4] {
        if self.cidr == 0 {
            return [0; 4];
        }

        let mask: u32 = u32::MAX << (32 - self.cidr);
        mask.to_be_bytes()
    }

    pub const fn to_bits(&self) -> u32 {
        u32::from_be_bytes(self.to_bytes())
    }

    pub const fn from_cidr(cidr: u8) -> Result<Self, &'static str> {
        if cidr > 32 {
            return Err("CIDR must be between 0 and 32!");
        }
        Ok(Self { cidr })
    }

    pub const fn from_integer(bytes: u32) -> Result<Self, &'static str> {
        const fn validate(bytes: u32) -> bool {
            if bytes == 0 || bytes == u32::MAX {
                return true;
            }

            let inverse = !bytes;
            inverse & inverse + 1 == 0
        }

        match bytes {
            b if validate(b) => Ok(Self {
                cidr: b.count_ones() as u8,
            }),
            _ => Err("Invalid bytes"),
        }
    }

    pub const fn from_bytes(bytes: [u8; 4]) -> Result<Self, &'static str> {
        Self::from_integer(u32::from_be_bytes(bytes))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Netmask;

pub enum Network {
    V4(Ipv4Network),
    V6(Ipv6Network),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Network {
    address: Ipv4Addr,
    netmask: Ipv4Netmask,
}

impl Ipv4Network {
    pub const fn address(&self) -> Ipv4Addr {
        self.address
    }

    pub const fn netmask(&self) -> Ipv4Netmask {
        self.netmask
    }

    pub const fn new(address: Ipv4Addr, netmask: Ipv4Netmask) -> Self {
        Self { address, netmask }
    }

    pub const fn network_addr(&self) -> Ipv4Addr {
        let addr_bits = self.address.to_bits();
        let mask_bits = self.netmask.to_bits();

        let network_bits = addr_bits & mask_bits;
        Ipv4Addr::from_bits(network_bits)
    }

    pub const fn broadcast_addr(&self) -> Ipv4Addr {
        let addr_bits = self.address.to_bits();
        let mask_bits = !self.netmask.to_bits();

        let network_bits = addr_bits | mask_bits;
        Ipv4Addr::from_bits(network_bits)
    }
}
pub struct Ipv6Network;
