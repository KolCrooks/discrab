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
    pub title: Option<String>,
    /// type of embed (always "rich" for webhook embeds)
    pub type_: Option<EmbedType>,
    /// description of embed
    pub description: Option<String>,
    /// URL of Embed
    pub url: Option<String>,
    /// timestamp of embed content
    pub timestamp: Option<String>,
    /// color code of the embed
    pub color: Option<Color>,
    /// footer information
    pub footer: Option<EmbedFooter>,
    /// image information
    pub image: Option<EmbedImage>,
    /// thumbnail information
    pub thumbnail: Option<EmbedThumbnail>,
    /// video information
    pub video: Option<EmbedVideo>,
    /// provider information
    pub provider: Option<EmbedProvider>,
    /// author information
    pub author: Option<EmbedAuthor>,
    /// fields information
    pub fields: Option<Vec<EmbedField>>,
}
