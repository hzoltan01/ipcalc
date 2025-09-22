use std::fmt::Debug;

pub mod v4;
pub mod v6;

pub use v4::*;
pub use v6::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Netmask {
    V4(Ipv4Netmask),
    V6(Ipv6Netmask),
}
