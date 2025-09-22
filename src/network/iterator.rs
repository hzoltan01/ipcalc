use std::net::Ipv4Addr;

pub struct NetworkIterator {
    current: Ipv4Addr,
    end: Ipv4Addr,
}

impl NetworkIterator {
    pub const fn new(first: Ipv4Addr, last: Ipv4Addr) -> Self {
        Self { current: first, end: last }
    }
}

impl Iterator for NetworkIterator {
    type Item = Ipv4Addr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            c if c == self.end => None,
            c => {
                self.current = Ipv4Addr::from_bits(self.current.to_bits() + 1);
                Some(c)
            }
        }
    }
}