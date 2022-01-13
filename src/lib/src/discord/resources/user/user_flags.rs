use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize};

bitflags! {
    /**
     * User Flags.
     * @docs <https://discord.com/developers/docs/resources/user#user-object-user-flags>
     */
    #[derive(Serialize)]
    pub struct UserFlags: u64 {
        /// None
        const NONE = 0;
        /// Discord Employee
        const STAFF = 1 << 0;
        /// Partnered Server Owner
        const PARTNER = 1 << 1;
        /// HypeSquad Events Coordinator
        const HYPESQUAD = 1 << 2;
        /// Bug Hunter Level 1
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// House Bravery Member
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance Member
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance Member
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro Supporter
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is a team
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug Hunter Level 2
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified Bot
        const VERIFIED_BOT = 1 << 16;
        /// Early Verified Bot Developer
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Discord Certified Moderator
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member list
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }

}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;

        UserFlags::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom(format!("Unexpected flags value {}", bits)))
    }
}
