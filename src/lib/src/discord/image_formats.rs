#![allow(clippy::upper_case_acronyms)]

use std::fmt::Display;

/**
 * Possible basic image formats. This excludes animated formats (GIF).
 * If you are looking for the one with GIFs, see ImageFormats::Animated
 */
pub enum Basic {
    PNG,
    JPG,
    WEBP,
}

/**
 * Possible image formats, includes GIF (as opposed to ImageFormats::Basic).
 */
pub enum Animated {
    PNG,
    JPG,
    WEBP,
    GIF,
}

impl Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Basic::PNG => write!(f, "png"),
            Basic::JPG => write!(f, "jpg"),
            Basic::WEBP => write!(f, "webp"),
        }
    }
}

impl Display for Animated {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Animated::PNG => write!(f, "png"),
            Animated::JPG => write!(f, "jpg"),
            Animated::WEBP => write!(f, "webp"),
            Animated::GIF => write!(f, "gif"),
        }
    }
}
