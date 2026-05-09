use serde::{de::Deserializer, Deserialize, Serialize};

use crate::models::{
    FamilyPlans, Frequency, Gender, GenderIdentity, RelationshipType, Seeking, ZodiacSign,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Filters {
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub gender: Vec<Gender>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub gender_identity: Vec<GenderIdentity>,

    pub age_min: Option<u16>,
    pub age_max: Option<u16>,

    pub height_min: Option<u8>,
    pub height_max: Option<u8>,

    pub distance: Option<u16>,

    #[serde(deserialize_with = "null_as_empty_vec")]
    pub zodiac_sign: Vec<ZodiacSign>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub seeking: Vec<Seeking>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub relationship_type: Vec<RelationshipType>,

    pub has_children: Option<bool>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub family_plans: Vec<FamilyPlans>,

    #[serde(deserialize_with = "null_as_empty_vec")]
    pub drinking: Vec<Frequency>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub smoking: Vec<Frequency>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub marijuana: Vec<Frequency>,
    #[serde(deserialize_with = "null_as_empty_vec")]
    pub drugs: Vec<Frequency>,

    pub image_count_min: Option<u8>,
}

fn null_as_empty_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default())
}
