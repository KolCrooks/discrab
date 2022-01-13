use crate::discord::resources::application::Application;

use crate::discord::resources::guild::guild::UnavailableGuild;

use crate::discord::resources::user::User;

/**
 * The ready event is dispatched when a client has completed the initial handshake with the gateway (for new sessions). The ready event can be the largest and most complex event the gateway will send, as it contains all the state required for a client to begin interacting with the rest of the platform.
 *
 * `guilds` are the guilds of which your bot is a member. They start out as unavailable when you connect to the gateway. As they become available, your bot will be notified via Guild Create events.
 *
 * @docs <https://discord.com/developers/docs/topics/gateway#ready>
 */
pub struct ReadyPayloadData {
    /// gateway version
    pub v: u64,
    /// information about the user including email
    pub user: User,
    /// the guilds the user is in
    pub guilds: Vec<UnavailableGuild>,
    /// used for resuming connections
    pub session_id: String,
    /// the shard information associated with this session, if sent when identifying
    pub shard: Option<(u64, u64)>,
    /// contains id and flags
    pub application: Option<Application>,
}
