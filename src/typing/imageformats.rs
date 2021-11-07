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

impl Basic {
    pub fn to_string(&self) -> String {
        match self {
            Basic::PNG => "png".to_string(),
            Basic::JPG => "jpg".to_string(),
            Basic::WEBP => "webp".to_string(),
        }
    }
}

impl Animated {
    pub fn to_string(&self) -> String {
        match self {
            Animated::PNG => "png".to_string(),
            Animated::JPG => "jpg".to_string(),
            Animated::WEBP => "webp".to_string(),
            Animated::GIF => "gif".to_string(),
        }
    }
}

// pub fn basic_to_str(format: Basic) -> &'static str {
//     match format {
//         Basic::PNG => "png",
//         Basic::JPG => "jpg",
//         Basic::WEBP => "webp",
//     }
// }

// pub fn animated_to_str(format: Animated) -> &'static str {
//     match format {
//         Animated::PNG => "png",
//         Animated::JPG => "jpg",
//         Animated::WEBP => "webp",
//         Animated::GIF => "gif",
//     }
// }