#![allow(non_camel_case_types)]

use super::context::Context;
use crate::core::interactions::handler::events::dispatch_payloads::{
    ChannelPinsUpdate, GuildBanAddRemove, GuildEmojisUpdate, GuildIntegrationsUpdate,
    GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate, GuildMembersChunk,
    GuildRoleCreateUpdateDelete, GuildScheduledEventUserAddRemove, GuildStickersUpdate,
    IntegrationCreateUpdate, IntegrationDelete, InviteCreate, InviteDelete, MessageDelete,
    MessageDeleteBulk, MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
    MessageReactionRemoveEmoji, ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate,
    TypingStart, VoiceServerUpdate, WebhooksUpdate,
};
use crate::core::interactions::typing::Interaction;
use crate::discord::gateway::presence::PresenceUpdate;
use crate::discord::resources::channel::message::Message;
use crate::discord::resources::channel::Channel;
use crate::discord::resources::guild::guild_object::{Guild, UnavailableGuild};
use crate::discord::resources::guild::stage_instance::StageInstance;
use crate::discord::resources::guild_scheduled_event::GuildScheduledEvent;
use crate::discord::resources::user::User;
use crate::discord::resources::voice::VoiceState;
use serde_json::Value;
use std::mem;

pub struct Observable<T: Clone> {
    subscribers: Vec<Box<dyn Fn(Context, T) + Send>>,
}

impl<T: Clone> Observable<T> {
    pub fn new() -> Self {
        Observable {
            subscribers: Vec::new(),
        }
    }

    pub async fn notify(&self, ctx: Context, data: T) {
        for listener in &self.subscribers {
            listener(ctx.clone(), data.clone());
        }
    }

    pub fn subscribe(&mut self, listener: &'static (dyn Fn(Context, T) + Send + Sync)) {
        self.subscribers.push(Box::new(listener));
    }
}

impl<T: Clone> Default for Observable<T> {
    fn default() -> Self {
        Observable::new()
    }
}

macro_rules! event_subscriptions {
    (
        $(#[$outer:meta])*
        pub struct $EventSubs:ident {
            $(
                $(#[$inner:meta])*
                const $Flag:ident: $x:ty = $EventName:expr;
            )+
        }
    ) => {
        $(#[$outer])*
        pub struct $EventSubs {
            $(
                $(#[$inner])*
                pub $Flag: Observable<$x>,
            )+
        }
        impl $EventSubs {
            pub fn new() -> Self {
                $EventSubs {
                    $(
                        $Flag: Observable::new(),
                    )+
                }
            }

            pub async fn route_event(&self, ctx: Context, event: String, data: Value) {
                match event.as_ref() {
                    $(
                        $EventName => {
                            let data = serde_json::from_value::<$x>(data).unwrap();
                            self.$Flag.notify(ctx, data).await;
                        }
                    )+
                    _ => {
                        println!("Unhandled event: {}", event);
                    }
                }
            }

            pub fn get_observable<T: Clone>(&mut self, event: String) -> &mut Observable<T> {
                let ptr: &mut bool = unsafe {
                    match event.as_str() {
                        $(
                            $EventName => mem::transmute(&mut self.$Flag),
                        )+
                        _ => panic!("Unhandled event: {}", event),
                    }
                };

                unsafe { mem::transmute(ptr) }
            }
        }

        impl Default for $EventSubs {
            fn default() -> Self {
                $EventSubs::new()
            }
        }

        pub enum Events {
            $(
                $Flag,
            )+
        }
     };
}

event_subscriptions! {
    /// * This is the event dispatcher for the bot.
    /// * It is responsible for routing events to the correct event handlers.
    pub struct EventDispatcher {
        //================
        //    Channels
        //================

        /// Sent when a new guild channel is created, relevant to the current user.
        const channel_create: Channel = "CHANNEL_CREATE";
        /// channel was updated
        /// This is not sent when the field last_message_id is altered. To keep track of the last_message_id changes,
        ///  you must listen for Message Create events.
        const channel_update: Channel = "CHANNEL_UPDATE";
        /// Sent when a channel relevant to the current user is deleted.
        const channel_delete: Channel = "CHANNEL_DELETE";
        /// message was pinned or unpinned
        const channel_pins_update: ChannelPinsUpdate = "CHANNEL_PINS_UPDATE";

        //================
        //    Threads
        //================

        /// Sent when a thread is created, relevant to the current user, or when the current user is added to a thread.
        /// When being added to an existing private thread, includes a thread member object.
        const thread_create: Channel = "THREAD_CREATE";
        /// Sent when a thread is updated. This is not sent when the field last_message_id is altered.
        /// To keep track of the last_message_id changes, you must listen for Message Create events.
        const thread_update: Channel = "THREAD_UPDATE";
        /// Sent when a thread relevant to the current user is deleted.
        /// The inner payload is a subset of the channel object, containing just the id, guild_id, parent_id, and type fields.
        /// //TODO make it so that the payload is a subset of the channel object
        const thread_delete: Channel = "THREAD_DELETE";
        /// thread list sync, contains all active threads in that channel
        /// Sent when the current user gains access to a channel.
        const thread_list_sync: ThreadListSync = "THREAD_LIST_SYNC";
        /// Sent when the thread member object for the current user is updated.
        /// The inner payload is a thread member object with an extra guild_id field.
        /// This event is documented for completeness, but unlikely to be used by most bots.
        /// For bots, this event largely is just a signal that you are a member of the thread.
        /// See the threads docs for more details.
        const thread_member_update: ThreadMemberUpdate = "THREAD_MEMBER_UPDATE";
        /// Sent when anyone is added to or removed from a thread.
        /// If the current user does not have the GUILD_MEMBERS Gateway Intent, then this event
        /// will only be sent if the current user was added to or removed from the thread.
        const thread_members_update: ThreadMembersUpdate = "THREAD_MEMBERS_UPDATE";
        /**
         * This event can be sent in three different scenarios:
         * 1. When a user is initially connecting, to lazily load and backfill information for all unavailable guilds sent in the Ready event. Guilds that are unavailable due to an outage will send a Guild Delete event.
         * 2. When a Guild becomes available again to the client.
         * 3. When the current user joins a new Guild.
         * The inner payload is a guild object, with all the extra fields specified.
         */
        const guild_create: Guild = "GUILD_CREATE";
        /// guild was updated
        const guild_update: Guild = "GUILD_UPDATE";
        /// Sent when a guild becomes or was already unavailable due to an outage,
        /// or when the user leaves or is removed from a guild.
        /// The inner payload is an unavailable guild object.
        /// If the unavailable field is not set, the user was removed from the guild.
        const guild_delete: UnavailableGuild = "GUILD_DELETE";
        /// user was banned from a guild
        const guild_ban_add: GuildBanAddRemove = "GUILD_BAN_ADD";
        /// user was unbanned from a guild
        const guild_ban_remove: GuildBanAddRemove = "GUILD_BAN_REMOVE";
        /// guild emojis were updated
        const guild_emojis_update: GuildEmojisUpdate = "GUILD_EMOJIS_UPDATE";
        /// guild stickers were updated
        const guild_stickers_update: GuildStickersUpdate = "GUILD_STICKERS_UPDATE";
        /// guild integration was updated
        const guild_integrations_update: GuildIntegrationsUpdate = "GUILD_INTEGRATIONS_UPDATE";
        /// new user joined a guild
        const guild_member_add: GuildMemberAdd = "GUILD_MEMBER_ADD";
        /// user was removed from a guild
        const guild_member_remove: GuildMemberRemove = "GUILD_MEMBER_REMOVE";
        /// guild member was updated
        const guild_member_update: GuildMemberUpdate = "GUILD_MEMBER_UPDATE";
        /// response to Request Guild Members
        const guild_members_chunk: GuildMembersChunk = "GUILD_MEMBERS_CHUNK";
        /// guild role was created
        const guild_role_create: GuildRoleCreateUpdateDelete = "GUILD_ROLE_CREATE";
        /// guild role was updated
        const guild_role_update: GuildRoleCreateUpdateDelete = "GUILD_ROLE_UPDATE";
        /// guild role was deleted
        const guild_role_delete: GuildRoleCreateUpdateDelete = "GUILD_ROLE_DELETE";
        /// guild scheduled event was created
        const guild_scheduled_event_create: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_CREATE";
        /// guild scheduled event was updated
        const guild_scheduled_event_update: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_UPDATE";
        /// guild scheduled event was deleted
        const guild_scheduled_event_delete: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_DELETE";
        /// user subscribed to a guild scheduled event
        const guild_scheduled_event_user_add: GuildScheduledEventUserAddRemove = "GUILD_SCHEDULED_EVENT_USER_ADD";
        /// user unsubscribed from a guild scheduled event
        const guild_scheduled_event_user_remove: GuildScheduledEventUserAddRemove = "GUILD_SCHEDULED_EVENT_USER_REMOVE";
        /// guild integration was created
        const integration_create: IntegrationCreateUpdate = "INTEGRATION_CREATE";
        /// guild integration was updated
        const integration_update: IntegrationCreateUpdate = "INTEGRATION_UPDATE";
        /// guild integration was deleted
        const integration_delete: IntegrationDelete = "INTEGRATION_DELETE";
        /// user used an interaction, such as an Application Command
        const interaction_create: Interaction = "INTERACTION_CREATE";
        /// invite to a channel was created
        const invite_create: InviteCreate = "INVITE_CREATE";
        /// invite to a channel was deleted
        const invite_delete: InviteDelete = "INVITE_DELETE";
        /// message was created
        const message_create: Message = "MESSAGE_CREATE";
        /// message was updated
        const message_update: Message = "MESSAGE_UPDATE";
        /// message was deleted
        const message_delete: MessageDelete = "MESSAGE_DELETE";
        /// multiple messages were deleted at once
        const message_delete_bulk: MessageDeleteBulk = "MESSAGE_DELETE_BULK";
        /// user reacted to a message
        const message_reaction_add: MessageReactionAdd = "MESSAGE_REACTION_ADD";
        /// user removed a reaction from a message
        const message_reaction_remove: MessageReactionRemove = "MESSAGE_REACTION_REMOVE";
        /// all reactions were explicitly removed from a message
        const message_reaction_remove_all: MessageReactionRemoveAll = "MESSAGE_REACTION_REMOVE_ALL";
        /// all reactions for a given emoji were explicitly removed from a message
        const message_reaction_remove_emoji: MessageReactionRemoveEmoji = "MESSAGE_REACTION_REMOVE_EMOJI";
        /// user was updated
        const presence_update: PresenceUpdate = "PRESENCE_UPDATE";
        /// stage instance was created
        const stage_instance_create: StageInstance = "STAGE_INSTANCE_CREATE";
        /// stage instance was deleted or closed
        const stage_instance_delete: StageInstance = "STAGE_INSTANCE_DELETE";
        /// stage instance was updated
        const stage_instance_update: StageInstance = "STAGE_INSTANCE_UPDATE";
        /// user started typing in a channel
        const typing_start: TypingStart = "TYPING_START";
        /// properties about the user changed
        const user_update: User = "USER_UPDATE";
        /// someone joined, left, or moved a voice channel
        const voice_state_update: VoiceState = "VOICE_STATE_UPDATE";
        /// guild's voice server was updated
        const voice_server_update: VoiceServerUpdate = "VOICE_SERVER_UPDATE";
        /// guild channel webhook was created, update, or deleted
        const webhooks_update: WebhooksUpdate = "WEBHOOKS_UPDATE";
    }
}
