use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, strum_macros::EnumIter)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "relationship_type"))]
pub enum RelationshipType {
    #[serde(rename = "monogamy")]
    #[cfg_attr(feature = "server", sqlx(rename = "monogamy"))]
    Monogamy,

    #[serde(rename = "non-monogamy")]
    #[cfg_attr(feature = "server", sqlx(rename = "non-monogamy"))]
    NonMonogamy,

    #[serde(rename = "figuring out my relationship type")]
    #[cfg_attr(feature = "server", sqlx(rename = "figuring out my relationship type"))]
    FiguringOutMyRelationshipType,
}

impl RelationshipType {
    pub const fn parts(&self) -> (&str, &str) {
        match self {
            Self::Monogamy => ("💍", "Monogamy"),
            Self::NonMonogamy => ("💞", "Non-monogamy"),
            Self::FiguringOutMyRelationshipType => ("🧭", "Figuring out my relationship type"),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let parts = s.split_once(" ")?;

        Self::iter().find(|g| g.parts() == parts)
    }
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (emoji, text) = self.parts();
        f.write_str(&format!("{emoji} {text}"))
    }
}
