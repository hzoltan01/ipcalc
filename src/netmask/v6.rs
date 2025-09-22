use std::fmt::{Display, Debug};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Netmask {
    cidr: u8,
}



impl Ipv6Netmask {
    pub const fn cidr(&self) -> u8 {
        self.cidr
    }

    pub const fn octets(&self) -> [u8; 16] {
        if self.cidr == 0 {
            return [0; 16];
        }

        let mask: u128 = u128::MAX << (128 - self.cidr);
        mask.to_be_bytes()
    }

    pub const fn hextets(&self) -> [u16; 8] {
        let octets = self.octets();
        let ptr: *const [u16; 8] = octets.as_ptr() as *const [u16; 8];

        // Safe because 8 * 16 = 16 * 8
        unsafe { *ptr }
    }

    pub const fn from_cidr(cidr: u8) -> Result<Self, &'static str> {
        if cidr > 128 {
            return Err("CIDR must be between 0 and 128!");
        }
        Ok(Self { cidr })
    }

    pub const fn from_bits(bytes: u128) -> Result<Self, &'static str> {
        const fn validate(bytes: u128) -> bool {
            if bytes == 0 || bytes == u128::MAX {
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

    pub const fn from_octets(bytes: [u8; 16]) -> Result<Self, &'static str> {
        Self::from_bits(u128::from_be_bytes(bytes))
    }

    pub const fn from_hextets(bytes: [u16; 8]) -> Result<Self, &'static str> {
        let ptr = bytes.as_ptr() as *const u128;
        // Safe because 8 * 16 = 16 * 8
        let bits = unsafe { *ptr };
        Self::from_bits(bits)
    }

    pub const fn host_num(&self) -> u128 {
        if self.cidr >= 127 {
            return 0;
        }

        2u128.pow(128 - self.cidr as u32) - 2
    }
}

impl TryFrom<u8> for Ipv6Netmask {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_cidr(value)
    }
}

impl TryFrom<u128> for Ipv6Netmask {
    type Error = &'static str;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::from_bits(value)
    }
}

impl TryFrom<[u8; 16]> for Ipv6Netmask {
    type Error = &'static str;

    fn try_from(value: [u8; 16]) -> Result<Self, Self::Error> {
        Self::from_octets(value)
    }
}

impl TryFrom<[u16; 8]> for Ipv6Netmask {
    type Error = &'static str;

    fn try_from(value: [u16; 8]) -> Result<Self, Self::Error> {
        Self::from_hextets(value)
    }
}

impl Display for Ipv6Netmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("/{}", self.cidr))
    }
}

impl Debug for Ipv6Netmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

