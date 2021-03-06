use crate::{
    guild::Member,
    id::{GuildId, UserId},
};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberChunk {
    pub guild_id: GuildId,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub members: HashMap<UserId, Member>,
    pub presences: HashMap<UserId, Member>,
}
