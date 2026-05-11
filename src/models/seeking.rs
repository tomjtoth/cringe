use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter, Display)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "seeking"))]
pub enum Seeking {
    #[strum(to_string = "🎉 Short-term fun")]
    #[serde(rename = "short-term fun")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term fun"))]
    ShortTermFun,

    #[strum(to_string = "🪄 Short-term, open to long")]
    #[serde(rename = "short-term, open to long")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term, open to long"))]
    ShortTermOpenToLong,

    #[strum(to_string = "🍷 Long-term, open to short")]
    #[serde(rename = "long-term, open to short")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term, open to short"))]
    LongTermOpenToShort,

    #[strum(to_string = "❤️ Long-term")]
    #[serde(rename = "long-term")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term"))]
    LongTerm,

    #[strum(to_string = "🤔 Still figuring it out")]
    #[serde(rename = "still figuring it out")]
    #[cfg_attr(feature = "server", sqlx(rename = "still figuring it out"))]
    StillFiguringItOut,
}

impl Seeking {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|g| g.to_string() == s)
    }
}
