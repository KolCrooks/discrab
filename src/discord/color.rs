use std::fmt::{Display, Error, Formatter};

use bitfield::bitfield;
use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Copy, Clone, Serialize, Deserialize)]
    pub struct Color(u32);
    pub r, _: 24, 16;
    pub g, _: 15, 8;
    pub b, _: 7, 0;
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#08x}", self.0)
    }
}
