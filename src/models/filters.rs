use serde::{Deserialize, Serialize};

use crate::models::{
    FamilyPlans, Frequency, Gender, GenderIdentity, RelationshipType, Seeking, ZodiacSign,
};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Filters {
    pub gender: Vec<Gender>,
    pub gender_identity: Vec<GenderIdentity>,

    pub age_min: Option<u16>,
    pub age_max: Option<u16>,

    pub height_min: Option<u16>,
    pub height_max: Option<u16>,

    pub distance: Option<u16>,

    pub zodiac_sign: Vec<ZodiacSign>,
    pub seeking: Vec<Seeking>,
    pub relationship_type: Vec<RelationshipType>,

    pub has_children: Option<bool>,
    pub family_plans: Vec<FamilyPlans>,

    pub drinking: Vec<Frequency>,
    pub smoking: Vec<Frequency>,
    pub marijuana: Vec<Frequency>,
    pub drugs: Vec<Frequency>,

    pub image_count_min: u8,
}
