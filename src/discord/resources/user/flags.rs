use bitfield::bitfield;

bitfield! {
    /**
     * User Flags.
     * @docshttps://discordapp.com/developers/docs/topics/gateway#user-object-user-flags
     */
    pub struct UserFlags(u64);

    u8;
    /// Discord Employee
    pub STAFF, _: 0, 1;
    /// Partnered Server Owner
    pub PARTNER, _: 1, 2;
    /// HypeSquad Events Coordinator
    pub HYPESQUAD, _: 2, 3;
    /// Bug Hunter Level 1
    pub BUG_HUNTER_LEVEL_1, _: 3, 4;
    /// House Bravery Member
    pub HYPESQUAD_ONLINE_HOUSE_1, _: 4, 5;
    /// House Brilliance Member
    pub HYPESQUAD_ONLINE_HOUSE_2, _: 5, 6;
    /// House Balance Member
    pub HYPESQUAD_ONLINE_HOUSE_3, _: 6, 7;
    /// Early Nitro Supporter
    pub PREMIUM_EARLY_SUPPORTER, _: 7, 8;
    /// * User is a team
    pub TEAM_PSEUDO_USER, _: 8, 9;
    /// Bug Hunter Level 2
    pub BUG_HUNTER_LEVEL_2, _: 9, 10;
    /// Verified Bot
    pub VERIFIED_BOT, _: 10, 11;
    /// Early Verified Bot Developer
    pub VERIFIED_DEVELOPER, _: 11, 12;
    /// Discord Certified Moderator
    pub CERTIFIED_MODERATOR, _: 12, 13;
    /// Bot uses only HTTP interactions and is shown in the online member list
    pub BOT_HTTP_INTERACTIONS, _: 13, 14;
}
