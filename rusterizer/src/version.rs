use std::fmt;
use std::convert::Into;

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone)]
pub struct Version {
    repr: u32,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        debug_assert!(major & (0b111111 << 10) == 0);
        debug_assert!(minor & (0b111111 << 10) == 0);
        debug_assert!(patch & (0b1111 << 12) == 0);
        let repr = ((major as u32) << 22) | ((minor as u32) << 12) | (patch as u32);
        Version { repr: repr }
    }

    pub fn from_repr(repr: u32) -> Self {
        Version { repr: repr }
    }

    pub fn get(&self) -> (u32, u32, u32) {
        let repr = self.repr;
        (
            (repr >> 22) & 0b1111111111,
            (repr >> 12) & 0b1111111111,
            repr & 0b111111111111,
        )
    }

    pub fn repr(&self) -> u32 {
        self.repr
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}

impl From<(u16, u16, u16)> for Version {
    fn from(t: (u16, u16, u16)) -> Version {
        Version::new(t.0, t.1, t.2)
    }
}
