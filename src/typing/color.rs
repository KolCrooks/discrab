use bitfield::bitfield;

bitfield! {
    pub struct Color(u32);
    pub R, _: 24, 16;
    pub G, _: 15, 8;
    pub B, _: 7, 0;
}

impl Color {
    pub fn to_string(&self) -> String {
        format!("{:#08x}", self.0)
    }
}