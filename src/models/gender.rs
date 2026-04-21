use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter, Display)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "gender", rename_all = "lowercase")
)]
pub enum Gender {
    #[strum(to_string = "♂️ Man")]
    Male,

    #[strum(to_string = "♀️ Woman")]
    Female,

    #[strum(to_string = "🌈 Non-binary")]
    #[serde(rename = "non-binary")]
    #[cfg_attr(feature = "server", sqlx(rename = "non-binary"))]
    NonBinary,
}

impl Gender {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|g| g.to_string() == s)
    }

    pub fn identities(&self) -> HashSet<GenderIdentity> {
        match self {
            Self::Male => HashSet::from([
                GenderIdentity::Demimale,
                GenderIdentity::GenderFluid,
                GenderIdentity::GenderQuestioning,
                GenderIdentity::Genderqueer,
                GenderIdentity::IntersexMan,
                GenderIdentity::TransMan,
                GenderIdentity::Transmasculine,
            ]),

            Self::Female => HashSet::from([
                GenderIdentity::Demifemale,
                GenderIdentity::GenderFluid,
                GenderIdentity::GenderQuestioning,
                GenderIdentity::Genderqueer,
                GenderIdentity::IntersexWoman,
                GenderIdentity::TransWoman,
                GenderIdentity::Transfeminine,
            ]),

            _ => {
                let hs: HashSet<_> = GenderIdentity::iter()
                    .filter(|gi| {
                        !matches!(gi, GenderIdentity::Demifemale | GenderIdentity::Demimale)
                    })
                    .collect();
                hs
            }
        }
    }
}

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "gender_identity", rename_all = "lowercase")
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter)]
pub enum GenderIdentity {
    #[serde(rename = "agender")]
    Agender,

    #[serde(rename = "bigender")]
    Bigender,

    #[serde(rename = "demimale")]
    Demimale,

    #[serde(rename = "demifemale")]
    Demifemale,

    #[strum(to_string = "Gender fluid")]
    #[serde(rename = "gender fluid")]
    #[cfg_attr(feature = "server", sqlx(rename = "gender fluid"))]
    GenderFluid,

    #[strum(to_string = "Gender nonconforming")]
    #[serde(rename = "gender nonconforming")]
    #[cfg_attr(feature = "server", sqlx(rename = "gender nonconforming"))]
    GenderNonconforming,

    #[strum(to_string = "Gender questioning")]
    #[serde(rename = "gender questioning")]
    #[cfg_attr(feature = "server", sqlx(rename = "gender questioning"))]
    GenderQuestioning,

    #[strum(to_string = "Gender variant")]
    #[serde(rename = "gender variant")]
    #[cfg_attr(feature = "server", sqlx(rename = "gender variant"))]
    GenderVariant,

    #[serde(rename = "genderqueer")]
    Genderqueer,

    #[serde(rename = "intersex")]
    Intersex,

    #[strum(to_string = "Intersex man")]
    #[serde(rename = "intersex man")]
    #[cfg_attr(feature = "server", sqlx(rename = "intersex man"))]
    IntersexMan,

    #[strum(to_string = "Intersex woman")]
    #[serde(rename = "intersex woman")]
    #[cfg_attr(feature = "server", sqlx(rename = "intersex woman"))]
    IntersexWoman,

    #[serde(rename = "neutrosis")]
    Neutrosis,

    #[serde(rename = "pangender")]
    Pangender,

    #[serde(rename = "polygender")]
    Polygender,

    #[strum(to_string = "Trans man")]
    #[serde(rename = "trans man")]
    #[cfg_attr(feature = "server", sqlx(rename = "trans man"))]
    TransMan,

    #[strum(to_string = "Trans woman")]
    #[serde(rename = "trans woman")]
    #[cfg_attr(feature = "server", sqlx(rename = "trans woman"))]
    TransWoman,

    #[serde(rename = "transfeminine")]
    Transfeminine,

    #[serde(rename = "transgender")]
    Transgender,

    #[serde(rename = "transmasculine")]
    Transmasculine,

    #[strum(to_string = "Two-spirit")]
    #[serde(rename = "two-spirit")]
    #[cfg_attr(feature = "server", sqlx(rename = "two-spirit"))]
    TwoSpirit,
}

impl GenderIdentity {
    pub fn from_str(s: &str) -> Option<Self> {
        Self::iter().find(|gi| gi.to_string() == s)
    }
}
