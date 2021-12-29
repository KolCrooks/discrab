#[derive(Clone, Default)]
pub struct Settings {
    debug_logging: bool,
}

impl Settings {
    pub fn set_debug_logging(&mut self, debug_logging: bool) {
        self.debug_logging = debug_logging;
    }
}
