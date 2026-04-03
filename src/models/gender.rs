use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "gender", rename_all = "lowercase")
)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// Display trait calls this too
    const fn label(&self) -> &'static str {
        match self {
            Gender::Male => "♂️ Man",
            Gender::Female => "♀️ Woman",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        [Gender::Male, Gender::Female]
            .into_iter()
            .find(|g| g.label() == s)
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}
