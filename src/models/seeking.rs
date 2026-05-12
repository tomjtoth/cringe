use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
    Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter, EnumProperty, Display,
)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "seeking"))]
pub enum Seeking {
    #[strum(
        to_string = "🎉 Short-term fun",
        props(glow = "text-shadow-violet-500")
    )]
    #[serde(rename = "short-term fun")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term fun"))]
    ShortTermFun,

    #[strum(
        to_string = "🪄 Short-term, open to long",
        props(glow = "text-shadow-amber-500")
    )]
    #[serde(rename = "short-term, open to long")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term, open to long"))]
    ShortTermOpenToLong,

    #[strum(
        to_string = "🍷 Long-term, open to short",
        props(glow = "text-shadow-rose-500")
    )]
    #[serde(rename = "long-term, open to short")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term, open to short"))]
    LongTermOpenToShort,

    #[strum(to_string = "❤️ Long-term", props(glow = "text-shadow-red-500"))]
    #[serde(rename = "long-term")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term"))]
    LongTerm,

    #[strum(
        to_string = "🤔 Still figuring it out",
        props(glow = "text-shadow-yellow-500")
    )]
    #[serde(rename = "still figuring it out")]
    #[cfg_attr(feature = "server", sqlx(rename = "still figuring it out"))]
    StillFiguringItOut,
}

impl Seeking {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|g| g.to_string() == s)
    }
}
