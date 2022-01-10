#[derive(Clone, Default)]
pub struct Settings {
    pub debug: bool,
}

impl Settings {
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}
