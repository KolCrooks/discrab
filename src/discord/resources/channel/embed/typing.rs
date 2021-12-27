use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 * Embed Types
 * Embed types are "loosely defined" and, for the most part, are not used by our clients for rendering. Embed attributes power what is rendered. Embed types should be considered deprecated and might be removed in a future API version.
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-types
 */
#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum EmbedType {
    /// generic embed rendered from embed attributes
    Rich = 0,
    /// image embed
    Image = 1,
    /// video embed
    Video = 2,
    /// animated gif image embed rendered as a video embed
    GifV = 3,
    /// article embed
    Article = 4,
    /// link embed
    Link = 5,
}

impl Display for EmbedType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            EmbedType::Rich => write!(f, "rich"),
            EmbedType::Image => write!(f, "image"),
            EmbedType::Video => write!(f, "video"),
            EmbedType::GifV => write!(f, "gifv"),
            EmbedType::Article => write!(f, "article"),
            EmbedType::Link => write!(f, "link"),
        }
    }
}

/**
 * Embed Thumbnail Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedThumbnail {
    /// Source url of thumbnail (only supports http(s) and attachments)
    pub url: String,
    /// A proxied url of the thumbnail
    pub proxy_url: Option<String>,
    /// Height of thumbnail
    pub height: Option<i32>,
    /// Width of thumbnail
    pub width: Option<i32>,
}

/**
 * Embed Image Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedImage {
    /// Source url of image (only supports http(s) and attachments)
    pub url: String,
    /// A proxied url of the image
    pub proxy_url: Option<String>,
    /// Height of image
    pub height: Option<i32>,
    /// Width of image
    pub width: Option<i32>,
}

/**
 * Embed Footer Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedFooter {
    /// Footer text
    pub text: String,
    /// Url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// Proxied url of footer icon
    pub proxy_icon_url: Option<String>,
}

/**
 * Embed Video Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedVideo {
    /// Source url of video (only supports http(s) and attachments)
    pub url: String,
    /// A proxied url of the video
    pub proxy_url: Option<String>,
    /// Height of video
    pub height: Option<i32>,
    /// Width of video
    pub width: Option<i32>,
}

/**
 * Embed Provider Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedProvider {
    /// Name of provider
    pub name: Option<String>,
    /// Url of provider
    pub url: Option<String>,
}

/**
 * Embed Author Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedAuthor {
    /// Name of author
    pub name: Option<String>,
    /// Url of author
    pub url: Option<String>,
    /// Url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// A proxied url of author icon
    pub proxy_icon_url: Option<String>,
}

/**
 * Embed Field Structure
 * @docs https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbedField {
    /// Name of field
    pub name: String,
    /// Value of field
    pub value: String,
    /// Whether or not this field should display inline
    pub inline: bool,
}
