use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum_macros::EnumIter)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "seeking"))]
pub enum Seeking {
    #[serde(rename = "short-term fun")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term fun"))]
    ShortTermFun,

    #[serde(rename = "short-term, open to long")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term, open to long"))]
    ShortTermOpenToLong,

    #[serde(rename = "long-term, open to short")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term, open to short"))]
    LongTermOpenToShort,

    #[serde(rename = "long-term")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term"))]
    LongTerm,

    #[serde(rename = "still figuring it out")]
    #[cfg_attr(feature = "server", sqlx(rename = "still figuring it out"))]
    StillFiguringItOut,
}

impl Seeking {
    pub const fn parts(&self) -> (&str, &str) {
        match self {
            Self::ShortTermFun => ("🎉", "Short-term fun"),
            Self::ShortTermOpenToLong => ("🪄", "Short-term, open to long"),
            Self::LongTermOpenToShort => ("🍷", "Long-term, open to short"),
            Self::LongTerm => ("❤️", "Long-term"),
            Self::StillFiguringItOut => ("🤔", "Still figuring it out"),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let parts = s.split_once(" ")?;

        Self::iter().find(|g| g.parts() == parts)
    }
}

impl std::fmt::Display for Seeking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (emoji, text) = self.parts();
        f.write_str(&format!("{emoji} {text}"))
    }
}
