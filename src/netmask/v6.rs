#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
