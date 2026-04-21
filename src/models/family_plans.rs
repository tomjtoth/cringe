use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "family_plans", rename_all = "lowercase")
)]
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Display, EnumIter)]
pub enum FamilyPlans {
    #[cfg_attr(feature = "server", sqlx(rename = "wants children"))]
    #[serde(rename = "wants children")]
    #[strum(to_string = "Wants")]
    WantsChildren,

    #[cfg_attr(feature = "server", sqlx(rename = "doesn't want children"))]
    #[serde(rename = "doesn't want children")]
    #[strum(to_string = "Doesn't want")]
    DoesntWantChildren,

    #[cfg_attr(feature = "server", sqlx(rename = "not sure yet"))]
    #[serde(rename = "not sure yet")]
    #[strum(to_string = "Not sure yet")]
    NotSureYet,
}

impl FamilyPlans {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|variant| variant.to_string() == s)
    }
}
