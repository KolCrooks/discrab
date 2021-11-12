use crate::discord::color::Color;

use super::typing::{
    EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedThumbnail, EmbedType,
    EmbedVideo,
};

// TODO create embed builder
/**
 * Embed Object
 * @docs https://discord.com/developers/docs/resources/channel#embed-object
 */
pub struct Embed {
    /// Title of Embed
    title: Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    type_: Option<EmbedType>,
    /// description of embed
    description: Option<String>,
    /// URL of Embed
    url: Option<String>,
    /// timestamp of embed content
    timestamp: Option<String>,
    /// color code of the embed
    color: Option<Color>,
    /// footer information
    footer: Option<EmbedFooter>,
    /// image information
    image: Option<EmbedImage>,
    /// thumbnail information
    thumbnail: Option<EmbedThumbnail>,
    /// video information
    video: Option<EmbedVideo>,
    /// provider information
    provider: Option<EmbedProvider>,
    /// author information
    author: Option<EmbedAuthor>,
    /// fields information
    fields: Option<Vec<EmbedField>>,
}
