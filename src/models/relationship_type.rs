use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter, Display)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "relationship_type"))]
pub enum RelationshipType {
    #[strum(to_string = "💍 Monogamy")]
    #[serde(rename = "monogamy")]
    #[cfg_attr(feature = "server", sqlx(rename = "monogamy"))]
    Monogamy,

    #[strum(to_string = "💞 Non-monogamy")]
    #[serde(rename = "non-monogamy")]
    #[cfg_attr(feature = "server", sqlx(rename = "non-monogamy"))]
    NonMonogamy,

    #[strum(to_string = "🧭 Figuring out my relationship type")]
    #[serde(rename = "figuring out my relationship type")]
    #[cfg_attr(feature = "server", sqlx(rename = "figuring out my relationship type"))]
    FiguringOutMyRelationshipType,
}

impl RelationshipType {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|g| g.to_string() == s)
    }
}
