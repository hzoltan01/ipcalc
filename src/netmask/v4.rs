use std::fmt::{Display, Debug};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Netmask {
    cidr: u8,
}

impl Ipv4Netmask {
    pub const fn cidr(&self) -> u8 {
        self.cidr
    }

    pub const fn octets(&self) -> [u8; 4] {
        if self.cidr == 0 {
            return [0; 4];
        }

        let mask: u32 = u32::MAX << (32 - self.cidr);
        mask.to_be_bytes()
    }

    pub const fn to_bits(&self) -> u32 {
        u32::from_be_bytes(self.octets())
    }

    pub const fn wildcard(&self) -> u32 {
        !self.to_bits()
    }

    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Result<Ipv4Netmask, &'static str> {
        Self::from_octets([a, b, c, d])
    }

    pub const fn from_cidr(cidr: u8) -> Result<Self, &'static str> {
        if cidr > 32 {
            return Err("CIDR must be between 0 and 32!");
        }
        Ok(Self { cidr })
    }

    pub const fn from_bits(bytes: u32) -> Result<Self, &'static str> {
        const fn validate(bytes: u32) -> bool {
            if bytes == 0 || bytes == u32::MAX {
                return true;
            }

            let inverse = !bytes;
            inverse & (inverse + 1) == 0
        }

        match bytes {
            b if validate(b) => Ok(Self {
                cidr: b.count_ones() as u8,
            }),
            _ => Err("Invalid bytes"),
        }
    }

    pub const fn from_octets(bytes: [u8; 4]) -> Result<Self, &'static str> {
        Self::from_bits(u32::from_be_bytes(bytes))
    }

    pub const fn host_num(&self) -> u64 {
        if self.cidr >= 31 {
            return 0;
        }

        2u64.pow(32 - self.cidr as u32) - 2
    }
}

impl TryFrom<u8> for Ipv4Netmask {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_cidr(value)
    }
}

impl TryFrom<u32> for Ipv4Netmask {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_bits(value)
    }
}

impl TryFrom<[u8; 4]> for Ipv4Netmask {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Self::from_octets(value)
    }
}

impl Display for Ipv4Netmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("/{}", self.cidr))
    }
}

impl Debug for Ipv4Netmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

