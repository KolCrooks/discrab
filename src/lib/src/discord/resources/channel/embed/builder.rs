use crate::discord::color::Color;

use super::{
    typing::{EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedThumbnail, EmbedType},
    Embed,
};

pub struct EmbedBuilder {
    embed: Embed,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        EmbedBuilder {
            embed: Embed {
                title: None,
                type_: Some(EmbedType::rich),
                description: None,
                url: None,
                timestamp: None,
                color: None,
                footer: None,
                image: None,
                thumbnail: None,
                video: None,
                provider: None,
                author: None,
                fields: None,
            },
        }
    }

    /// Sets the title of the embed.
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.embed.title = Some(title.to_string());
        self
    }

    /// Sets the description of the embed.
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.embed.description = Some(description.to_string());
        self
    }

    /// Sets the color of the embed
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.embed.color = Some(color);
        self
    }

    /// Add a field to the embed.
    pub fn add_field(&mut self, field: EmbedField) -> &mut Self {
        match self.embed.fields {
            Some(ref mut fields) => fields.push(field),
            None => self.embed.fields = Some(vec![field]),
        }
        self
    }

    /// Set the footer portion of the embed.
    pub fn set_footer(&mut self, footer: EmbedFooter) -> &mut Self {
        self.embed.footer = Some(footer);
        self
    }

    /// Set the image portion of the embed.
    pub fn set_image(&mut self, img: EmbedImage) -> &mut Self {
        self.embed.image = Some(img);
        self
    }

    /// Set the thumbnail portion of the embed.
    pub fn set_thumbnail(&mut self, thumbnail: EmbedThumbnail) -> &mut Self {
        self.embed.thumbnail = Some(thumbnail);
        self
    }

    /// Sets the author for the embed
    pub fn set_author(&mut self, author: EmbedAuthor) -> &mut Self {
        self.embed.author = Some(author);
        self
    }

    /// Builds the embed.
    pub fn build(self) -> Embed {
        self.embed
    }
}

impl Default for EmbedBuilder {
    fn default() -> Self {
        EmbedBuilder::new()
    }
}
