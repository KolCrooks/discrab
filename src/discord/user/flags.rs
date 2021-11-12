/**
 * User Flags. See this page for more information: https://discordapp.com/developers/docs/topics/gateway#user-object-user-flags
 */
pub enum UserFlags {
    /**
     * None
     */
    NONE = 0,
    /**
     * Discord Employee
     */
    STAFF = 1,
    /**
     * Partnered Server Owner
     */
    PARTNER = 1 << 1,
    /**
     * HypeSquad Events Coordinator
     */
    HYPESQUAD = 1 << 2,
    /**
     * Bug Hunter Level 1
     */
    BUG_HUNTER_LEVEL_1 = 1 << 3,
    /**
     * House Bravery Member
     */
    HYPESQUAD_ONLINE_HOUSE_1 = 1 << 4,
    /**
     * House Brilliance Member
     */
    HYPESQUAD_ONLINE_HOUSE_2 = 1 << 5,
    /**
     * House Balance Member
     */
    HYPESQUAD_ONLINE_HOUSE_3 = 1 << 6,
    /**
     * Early Nitro Supporter
     */
    PREMIUM_EARLY_SUPPORTER = 1 << 7,
    /**
     * User is a team
     */
    TEAM_PSEUDO_USER = 1 << 8,
    /**
     * Bug Hunter Level 2
     */
    BUG_HUNTER_LEVEL_2 = 1 << 9,
    /**
     * Verified Bot
     */
    VERIFIED_BOT = 1 << 10,
    /**
     * Early Verified Bot Developer
     */
    VERIFIED_DEVELOPER = 1 << 11,
    /**
     * Discord Certified Moderator
     */
    CERTIFIED_MODERATOR = 1 << 12,
    /**
     * Bot uses only HTTP interactions and is shown in the online member list
     */
    BOT_HTTP_INTERACTIONS = 1 << 13,
}
