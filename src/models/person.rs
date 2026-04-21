use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{FamilyPlans, Gender, Image, RelationshipType, Seeking, ZodiacSign};

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    #[cfg_attr(feature = "server", sqlx(rename = "yes"))]
    YesPlease,
}

impl Frequency {
    const fn label(&self) -> &str {
        match self {
            Self::Never => "Never",
            Self::Rarely => "Rarely",
            Self::Often => "Often",
            Self::YesPlease => "Yes, please!",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        [Self::Never, Self::Rarely, Self::Often, Self::YesPlease]
            .into_iter()
            .find(|freq| freq.label() == s)
    }
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
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
pub struct Person {
    #[serde(default)]
    pub id: Option<i32>,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    pub gender: Gender,
    pub born: Option<NaiveDate>,
    #[serde(default)]
    pub age: Option<u16>,
    pub zodiac_sign: Option<ZodiacSign>,

    pub height: i16,
    #[serde(default)]
    pub education: Option<String>,
    #[serde(default)]
    pub occupation: Option<String>,

    // location is a human-readable city/area label shown to clients
    #[serde(default)]
    pub location: Option<String>,

    // gps coordinates are server-side source data for distance calculations
    #[serde(default)]
    pub gps: Option<Gps>,

    // u16::MAX = 65_535 vs. 40_075 largest Earth circumference
    #[serde(default)]
    pub distance: Option<u16>,

    #[serde(default)]
    pub hometown: Option<String>,
    #[serde(default)]
    pub seeking: Option<Seeking>,
    #[serde(default)]
    pub relationship_type: Option<RelationshipType>,

    pub has_children: Option<bool>,
    pub family_plans: Option<FamilyPlans>,

    pub drinking: Option<Frequency>,
    pub smoking: Option<Frequency>,
    pub marijuana: Option<Frequency>,
    pub drugs: Option<Frequency>,

    pub prompts: Vec<Prompt>,
    pub images: Vec<Image>,
}

#[cfg(feature = "server")]
impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Person {
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
        let born = try_opt!(NaiveDate, "born");
        let age = try_opt!(i32, "age").map(|a| a as u16);
        let zodiac_sign = try_opt!(ZodiacSign, "zodiac_sign");
        let height: i16 = row.try_get("height")?;

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

        let drinking = try_opt!(Frequency, "habits_drinking");
        let smoking = try_opt!(Frequency, "habits_smoking");
        let marijuana = try_opt!(Frequency, "habits_marijuana");
        let drugs = try_opt!(Frequency, "habits_drugs");

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

        Ok(Person {
            id,
            name,
            email,
            gender,
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

impl Person {
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
