use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{
    FamilyPlans, Frequency, Gender, GenderIdentity, Image, RelationshipType, Seeking, ZodiacSign,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "decision", rename_all = "lowercase")
)]
pub enum Decision {
    Like,
    Skip,
}

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Prompt {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub position: Option<i16>,
    pub title: String,
    pub body: String,
}

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gps {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub id: Option<i32>,
    pub name: String,
    pub email: Option<String>,
    pub gender: Gender,
    pub gender_identity: Option<GenderIdentity>,
    pub born: Option<NaiveDate>,

    pub age: Option<u16>,
    pub zodiac_sign: Option<ZodiacSign>,

    pub height: u8,
    pub education: Option<String>,
    pub occupation: Option<String>,

    // location is a human-readable city/area label shown to clients
    pub location: Option<String>,

    // gps coordinates are server-side source data for distance calculations
    pub gps: Option<Gps>,

    // u16::MAX = 65_535 vs. 40_075 largest Earth circumference
    pub distance: Option<u16>,

    pub hometown: Option<String>,
    pub seeking: Option<Seeking>,
    pub relationship_type: Option<RelationshipType>,

    pub has_children: Option<bool>,
    pub family_plans: Option<FamilyPlans>,

    pub drinking: Option<Frequency>,
    pub smoking: Option<Frequency>,
    pub marijuana: Option<Frequency>,
    pub drugs: Option<Frequency>,

    #[serde(default)]
    pub prompts: Vec<Prompt>,
    #[serde(default)]
    pub images: Vec<Image>,
}

#[cfg(feature = "server")]
impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Profile {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::{types::Json, Row};

        macro_rules! try_opt {
            ($ty:ty, $name:expr) => {
                match row.try_get::<Option<$ty>, _>($name) {
                    Ok(v) => v,
                    Err(sqlx::Error::ColumnNotFound(_)) => None,
                    Err(e) => return Err(e),
                }
            };
        }

        let id = try_opt!(i32, "id");
        let email = try_opt!(String, "email");

        let name: String = row.try_get("name")?;
        let gender: Gender = row.try_get("gender")?;
        let gender_identity = try_opt!(GenderIdentity, "gender_identity");
        let born = try_opt!(NaiveDate, "born");
        let age = try_opt!(i32, "age").map(|a| a as u16);
        let zodiac_sign = try_opt!(ZodiacSign, "zodiac_sign");
        let height: i16 = row.try_get("height")?;
        let height = height as u8;

        let education = try_opt!(String, "education");
        let occupation = try_opt!(String, "occupation");

        let location = try_opt!(String, "location");

        let gps = match row.try_get::<Option<Gps>, _>("gps") {
            Ok(v) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => None,
            Err(e) => return Err(e),
        };

        let distance = try_opt!(f64, "distance").map(|d| d.round() as u16);

        let hometown = try_opt!(String, "hometown");
        let seeking = try_opt!(Seeking, "seeking");
        let relationship_type = try_opt!(RelationshipType, "relationship_type");

        let has_children = try_opt!(bool, "has_children");
        let family_plans = try_opt!(FamilyPlans, "family_plans");

        let drinking = try_opt!(Frequency, "drinking");
        let smoking = try_opt!(Frequency, "smoking");
        let marijuana = try_opt!(Frequency, "marijuana");
        let drugs = try_opt!(Frequency, "drugs");

        let prompts = match row.try_get::<Json<Vec<Prompt>>, _>("prompts") {
            Ok(Json(v)) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => Vec::new(),
            Err(e) => return Err(e),
        };

        let images = match row.try_get::<Json<Vec<Image>>, _>("images") {
            Ok(Json(v)) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => Vec::new(),
            Err(e) => return Err(e),
        };

        Ok(Profile {
            id,
            name,
            email,
            gender,
            gender_identity,
            born,
            age,
            zodiac_sign,
            height,
            education,
            occupation,
            location,
            gps,
            distance,
            hometown,
            seeking,
            relationship_type,

            has_children,
            family_plans,

            drinking,
            smoking,
            marijuana,
            drugs,

            prompts,
            images,
        })
    }
}

impl Profile {
    pub fn age(&self) -> Option<u16> {
        if self.age.is_some() {
            return self.age;
        }

        self.born.map(|born| {
            let today = Utc::now().date_naive();
            today.years_since(born).unwrap() as u16
        })
    }

    pub fn zodiac_sign(&self) -> Option<ZodiacSign> {
        if self.zodiac_sign.is_some() {
            return self.zodiac_sign;
        }

        self.born.map(ZodiacSign::from_date)
    }

    pub fn distance(&self) -> Option<String> {
        self.distance.map(|x| {
            let (emoji, d) = match x {
                0..=3 => ("🥾", x),
                4..=10 => ("🚲", x),
                11..=30 => ("🚗", x),
                31..=120 => ("🚂", x),
                _ => ("✈️", x),
            };
            format!("{emoji} ~{d}km away")
        })
    }
}
