use std::fmt::{Display, Error, Formatter};

use bitfield::bitfield;

bitfield! {
    pub struct Color(u32);
    pub R, _: 24, 16;
    pub G, _: 15, 8;
    pub B, _: 7, 0;
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(&mut f, "{:#08x}", self.0)
    }
}
