use serde::Serialize;

use crate::{
    api::Snowflake,
    core::interactions::{message::MessageComponent, typing::AllowedMentions},
};

use super::{
    embed::{Embed, EmbedBuilder},
    typing::MessageReference,
};

/**
 * Used to create messages that can be sent in a channel.
 */
#[derive(Serialize)]
pub struct MessageBuilder {
    /// The message content (up to 2000 characters)
    content: Option<String>,
    /// if this is a TTS message
    tts: Option<bool>,
    /// embedded rich content (up to 6000 characters)
    embeds: Option<Vec<Embed>>,
    /// allowed mentions for the message
    allowed_mentions: Option<AllowedMentions>,
    /// include to make your message a reply
    message_reference: Option<MessageReference>,
    /// the components to include with the message
    components: Option<Vec<MessageComponent>>,
    /// IDs of up to 3 stickers in the server to send in the message
    sticker_ids: Option<Vec<Snowflake>>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            content: None,
            tts: None,
            embeds: None,
            allowed_mentions: None,
            message_reference: None,
            components: None,
            sticker_ids: None,
        }
    }

    /// Add string content to the message
    #[must_use]
    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    /// Make the message TTS
    #[must_use]
    pub fn set_tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }

    /// Add an embed to the message
    #[must_use]
    pub fn add_embed<F: Fn(&mut EmbedBuilder)>(mut self, embed_fn: F) -> Self {
        match self.embeds {
            Some(ref mut embeds) => {
                let mut builder = EmbedBuilder::new();
                embed_fn(&mut builder);
                embeds.push(builder.build());
            }
            None => {
                let mut embeds = Vec::new();
                let mut builder = EmbedBuilder::new();
                embed_fn(&mut builder);
                embeds.push(builder.build());
                self.embeds = Some(embeds);
            }
        }
        self
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
