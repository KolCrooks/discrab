#![allow(non_snake_case)]

use super::abstraction_traits::CommandArg;
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
use crate::discord::resources::channel::{message::Message, Channel};
use crate::discord::resources::guild::guild_object::{Guild, UnavailableGuild};
use crate::discord::resources::guild::stage_instance::StageInstance;
use crate::discord::resources::guild_scheduled_event::GuildScheduledEvent;
use crate::discord::resources::user::User;
use crate::discord::resources::voice::VoiceState;
use crate::util::logger::print_debug;
use serde_json::Value;
use std::mem;

pub struct Observable<'a, T: Clone + CommandArg> {
    subscribers: Vec<&'a (dyn Fn(Context, T) + Send)>,
}

impl<'a, T: Clone + CommandArg> Observable<'a, T> {
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

    pub fn subscribe(&mut self, listener: &'a (dyn Fn(Context, T) + Send + Sync)) {
        self.subscribers.push(listener);
    }
}

impl<'a, T: Clone + CommandArg> Default for Observable<'a, T> {
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
        pub struct $EventSubs<'a>{
            $(
                $(#[$inner])*
                pub $Flag: Observable<'a, $x>,
            )+
        }
        impl<'a> $EventSubs<'a>{
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
                        print_debug("EVENT_HANDLER", format!("Unhandled event: {}", event));
                    }
                }
            }

            pub fn get_observable<T: Clone + CommandArg>(&mut self, event: Events, type_str: &str) -> &mut Observable<T> {
                unsafe {
                    match event {
                        $(
                            Events::$Flag => {
                                if stringify!($x) != type_str {
                                    panic!("Event type mismatch! Expected type: `{}`, recieved: `{}`", stringify!($x), type_str);
                                }
                                mem::transmute(&mut self.$Flag)
                            },
                        )+
                    }
                }
            }

            pub fn get_observable_no_check<T: Clone + CommandArg>(&mut self, event: Events) -> &mut Observable<T> {
                unsafe {
                    match event {
                        $(
                            Events::$Flag => {
                                mem::transmute(&mut self.$Flag)
                            },
                        )+
                    }
                }
            }
        }

        impl<'a> Default for $EventSubs<'a>{
            fn default() -> Self {
                $EventSubs::new()
            }
        }

        pub enum Events {
            $(
                #[doc="$x"]
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
        const ChannelCreate: Channel = "CHANNEL_CREATE";
        /// channel was updated
        /// This is not sent when the field last_message_id is altered. To keep track of the last_message_id changes,
        ///  you must listen for Message Create events.
        const ChannelUpdate: Channel = "CHANNEL_UPDATE";
        /// Sent when a channel relevant to the current user is deleted.
        const ChannelDelete: Channel = "CHANNEL_DELETE";
        /// message was pinned or unpinned
        const ChannelPinsUpdate: ChannelPinsUpdate = "CHANNEL_PINS_UPDATE";

        //================
        //    Threads
        //================

        /// Sent when a thread is created, relevant to the current user, or when the current user is added to a thread.
        /// When being added to an existing private thread, includes a thread member object.
        const ThreadCreate: Channel = "THREAD_CREATE";
        /// Sent when a thread is updated. This is not sent when the field last_message_id is altered.
        /// To keep track of the last_message_id changes, you must listen for Message Create events.
        const ThreadUpdate: Channel = "THREAD_UPDATE";
        /// Sent when a thread relevant to the current user is deleted.
        /// The inner payload is a subset of the channel object, containing just the id, guild_id, parent_id, and type fields.
        /// //TODO make it so that the payload is a subset of the channel object
        const ThreadDelete: Channel = "THREAD_DELETE";
        /// thread list sync, contains all active threads in that channel
        /// Sent when the current user gains access to a channel.
        const ThreadListSync: ThreadListSync = "THREAD_LIST_SYNC";
        /// Sent when the thread member object for the current user is updated.
        /// The inner payload is a thread member object with an extra guild_id field.
        /// This event is documented for completeness, but unlikely to be used by most bots.
        /// For bots, this event largely is just a signal that you are a member of the thread.
        /// See the threads docs for more details.
        const ThreadMemberUpdate: ThreadMemberUpdate = "THREAD_MEMBER_UPDATE";
        /// Sent when anyone is added to or removed from a thread.
        /// If the current user does not have the GUILD_MEMBERS Gateway Intent, then this event
        /// will only be sent if the current user was added to or removed from the thread.
        const ThreadMembersUpdate: ThreadMembersUpdate = "THREAD_MEMBERS_UPDATE";
        /**
         * This event can be sent in three different scenarios:
         * 1. When a user is initially connecting, to lazily load and backfill information for all unavailable guilds sent in the Ready event. Guilds that are unavailable due to an outage will send a Guild Delete event.
         * 2. When a Guild becomes available again to the client.
         * 3. When the current user joins a new Guild.
         * The inner payload is a guild object, with all the extra fields specified.
         */
        const GuildCreate: Guild = "GUILD_CREATE";
        /// guild was updated
        const GuildUpdate: Guild = "GUILD_UPDATE";
        /// Sent when a guild becomes or was already unavailable due to an outage,
        /// or when the user leaves or is removed from a guild.
        /// The inner payload is an unavailable guild object.
        /// If the unavailable field is not set, the user was removed from the guild.
        const GuildDelete: UnavailableGuild = "GUILD_DELETE";
        /// user was banned from a guild
        const GuildBanAdd: GuildBanAddRemove = "GUILD_BAN_ADD";
        /// user was unbanned from a guild
        const GuildBanRemove: GuildBanAddRemove = "GUILD_BAN_REMOVE";
        /// guild emojis were updated
        const GuildEmojisUpdate: GuildEmojisUpdate = "GUILD_EMOJIS_UPDATE";
        /// guild stickers were updated
        const GuildStickersUpdate: GuildStickersUpdate = "GUILD_STICKERS_UPDATE";
        /// guild integration was updated
        const GuildIntegrationsUpdate: GuildIntegrationsUpdate = "GUILD_INTEGRATIONS_UPDATE";
        /// new user joined a guild
        const GuildMemberAdd: GuildMemberAdd = "GUILD_MEMBER_ADD";
        /// user was removed from a guild
        const GuildMemberRemove: GuildMemberRemove = "GUILD_MEMBER_REMOVE";
        /// guild member was updated
        const GuildMemberUpdate: GuildMemberUpdate = "GUILD_MEMBER_UPDATE";
        /// response to Request Guild Members
        const GuildMembersChunk: GuildMembersChunk = "GUILD_MEMBERS_CHUNK";
        /// guild role was created
        const GuildRoleCreate: GuildRoleCreateUpdateDelete = "GUILD_ROLE_CREATE";
        /// guild role was updated
        const GuildRoleUpdate: GuildRoleCreateUpdateDelete = "GUILD_ROLE_UPDATE";
        /// guild role was deleted
        const GuildRoleDelete: GuildRoleCreateUpdateDelete = "GUILD_ROLE_DELETE";
        /// guild scheduled event was created
        const GuildScheduledEventCreate: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_CREATE";
        /// guild scheduled event was updated
        const GuildScheduledEventUpdate: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_UPDATE";
        /// guild scheduled event was deleted
        const GuildScheduledEventDelete: GuildScheduledEvent = "GUILD_SCHEDULED_EVENT_DELETE";
        /// user subscribed to a guild scheduled event
        const GuildScheduledEventUserAdd: GuildScheduledEventUserAddRemove = "GUILD_SCHEDULED_EVENT_USER_ADD";
        /// user unsubscribed from a guild scheduled event
        const GuildScheduledEventUserRemove: GuildScheduledEventUserAddRemove = "GUILD_SCHEDULED_EVENT_USER_REMOVE";
        /// guild integration was created
        const IntegrationCreate: IntegrationCreateUpdate = "INTEGRATION_CREATE";
        /// guild integration was updated
        const IntegrationUpdate: IntegrationCreateUpdate = "INTEGRATION_UPDATE";
        /// guild integration was deleted
        const IntegrationDelete: IntegrationDelete = "INTEGRATION_DELETE";
        /// user used an interaction, such as an Application Command
        const InteractionCreate: Interaction = "INTERACTION_CREATE";
        /// invite to a channel was created
        const InviteCreate: InviteCreate = "INVITE_CREATE";
        /// invite to a channel was deleted
        const InviteDelete: InviteDelete = "INVITE_DELETE";
        /// message was created
        const MessageCreate: Message = "MESSAGE_CREATE";
        /// message was updated
        const MessageUpdate: Message = "MESSAGE_UPDATE";
        /// message was deleted
        const MessageDelete: MessageDelete = "MESSAGE_DELETE";
        /// multiple messages were deleted at once
        const MessageDeleteBulk: MessageDeleteBulk = "MESSAGE_DELETE_BULK";
        /// user reacted to a message
        const MessageReactionAdd: MessageReactionAdd = "MESSAGE_REACTION_ADD";
        /// user removed a reaction from a message
        const MessageReactionRemove: MessageReactionRemove = "MESSAGE_REACTION_REMOVE";
        /// all reactions were explicitly removed from a message
        const MessageReactionRemoveAll: MessageReactionRemoveAll = "MESSAGE_REACTION_REMOVE_ALL";
        /// all reactions for a given emoji were explicitly removed from a message
        const MessageReactionRemoveEmoji: MessageReactionRemoveEmoji = "MESSAGE_REACTION_REMOVE_EMOJI";
        /// user was updated
        const PresenceUpdate: PresenceUpdate = "PRESENCE_UPDATE";
        /// stage instance was created
        const StageInstanceCreate: StageInstance = "STAGE_INSTANCE_CREATE";
        /// stage instance was deleted or closed
        const StageInstanceDelete: StageInstance = "STAGE_INSTANCE_DELETE";
        /// stage instance was updated
        const StageInstanceUpdate: StageInstance = "STAGE_INSTANCE_UPDATE";
        /// user started typing in a channel
        const TypingStart: TypingStart = "TYPING_START";
        /// properties about the user changed
        const UserUpdate: User = "USER_UPDATE";
        /// someone joined, left, or moved a voice channel
        const VoiceStateUpdate: VoiceState = "VOICE_STATE_UPDATE";
        /// guild's voice server was updated
        const VoiceServerUpdate: VoiceServerUpdate = "VOICE_SERVER_UPDATE";
        /// guild channel webhook was created, update, or deleted
        const WebhooksUpdate: WebhooksUpdate = "WEBHOOKS_UPDATE";
    }
}
