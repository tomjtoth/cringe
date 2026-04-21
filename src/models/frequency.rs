use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter, Display)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "frequency", rename_all = "lowercase")
)]
pub enum Frequency {
    Never,
    Rarely,
    Often,

    #[serde(rename = "yes")]
    #[strum(to_string = "Yes, please!")]
    #[cfg_attr(feature = "server", sqlx(rename = "yes"))]
    YesPlease,
}

impl Frequency {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|freq| freq.to_string() == s)
    }
}
