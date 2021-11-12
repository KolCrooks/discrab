use bitfield::bitfield;

bitfield! {
    /**
     * User Flags.
     * @docshttps://discordapp.com/developers/docs/topics/gateway#user-object-user-flags
     */
    pub struct UserFlags(u64);

    u8;
    /// Discord Employee
    pub staff, _: 0, 1;
    /// Partnered Server Owner
    pub partner, _: 1, 2;
    /// HypeSquad Events Coordinator
    pub hypesquad, _: 2, 3;
    /// Bug Hunter Level 1
    pub bug_hunter_level_1, _: 3, 4;
    /// House Bravery Member
    pub hypesquad_online_house_1, _: 4, 5;
    /// House Brilliance Member
    pub hypesquad_online_house_2, _: 5, 6;
    /// House Balance Member
    pub hypesquad_online_house_3, _: 6, 7;
    /// Early Nitro Supporter
    pub premium_early_supporter, _: 7, 8;
    /// * User is a team
    pub team_pseudo_user, _: 8, 9;
    /// Bug Hunter Level 2
    pub bug_hunter_level_2, _: 9, 10;
    /// Verified Bot
    pub verified_bot, _: 10, 11;
    /// Early Verified Bot Developer
    pub verified_developer, _: 11, 12;
    /// Discord Certified Moderator
    pub certified_moderator, _: 12, 13;
    /// Bot uses only HTTP interactions and is shown in the online member list
    pub bot_http_interactions, _: 13, 14;
}
