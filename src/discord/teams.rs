use super::{resources::user::User, snowflake::Snowflake};

/**
 * Team Object
 * @docs https://discord.com/developers/docs/topics/teams#data-models-team-object
 */
pub struct Team {
    /// a hash of the image of the team's icon
    pub icon: Option<String>,
    /// the unique id of the team
    pub id: Snowflake,
    /// The members of the team
    pub members: Vec<TeamMember>,
    /// the name of the team
    pub name: String,
    /// the user id of the current team owner
    pub owner_user_id: String,
}

/**
 * Team Member Object
 * @docs https://discord.com/developers/docs/topics/teams#data-models-team-member-object
 */
pub struct TeamMember {
    /// the user's membership state on the team
    pub membership_state: MembershipState,
    /// will always be ["*"]
    pub permissions: Vec<String>,
    /// the id of the parent team of which they are a member
    pub team_id: Snowflake,
    /// the avatar, discriminator, id, and username of the user
    pub user: User,
}

/**
 * Membership State Enum
 * @docs https://discord.com/developers/docs/topics/teams#data-models-team-member-object-membership-state
 */
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}
