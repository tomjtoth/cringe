use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::types::Json;

use crate::models::{
    gender::Gender,
    image::{Image, TImages},
    kids::TKids,
    relationship_type::RelationshipType,
    seeking::Seeking,
};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Habits {
    pub drinking: Option<Frequency>,
    pub smoking: Option<Frequency>,
    pub marijuana: Option<Frequency>,
    pub drugs: Option<Frequency>,
}

#[cfg(feature = "server")]
type THabits = Json<Habits>;

#[cfg(not(feature = "server"))]
type THabits = Habits;

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

#[cfg(feature = "server")]
type TPrompts = Json<Vec<Prompt>>;
#[cfg(not(feature = "server"))]
type TPrompts = Vec<Prompt>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "zodiac_sign", rename_all = "lowercase")
)]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl ZodiacSign {
    fn from_date(date: NaiveDate) -> Self {
        let month = date.month();
        let day = date.day();

        match (month, day) {
            (3, 21..) | (4, ..=19) => Self::Aries,
            (4, 20..) | (5, ..=20) => Self::Taurus,
            (5, 21..) | (6, ..=20) => Self::Gemini,
            (6, 21..) | (7, ..=22) => Self::Cancer,
            (7, 23..) | (8, ..=22) => Self::Leo,
            (8, 23..) | (9, ..=22) => Self::Virgo,
            (9, 23..) | (10, ..=22) => Self::Libra,
            (10, 23..) | (11, ..=21) => Self::Scorpio,
            (11, 22..) | (12, ..=21) => Self::Sagittarius,
            (12, 22..) | (1, ..=19) => Self::Capricorn,
            (1, 20..) | (2, ..=18) => Self::Aquarius,
            (2, 19..) | (3, ..=20) => Self::Pisces,
            _ => unreachable!("NaiveDate always has a valid month/day"),
        }
    }
}

impl std::fmt::Display for ZodiacSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Aries => "♈ Aries",
            Self::Taurus => "♉ Taurus",
            Self::Gemini => "♊ Gemini",
            Self::Cancer => "♋ Cancer",
            Self::Leo => "♌ Leo",
            Self::Virgo => "♍ Virgo",
            Self::Libra => "♎ Libra",
            Self::Scorpio => "♏ Scorpio",
            Self::Sagittarius => "♐ Sagittarius",
            Self::Capricorn => "♑ Capricorn",
            Self::Aquarius => "♒ Aquarius",
            Self::Pisces => "♓ Pisces",
        };

        f.write_str(label)
    }
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
    #[serde(default)]
    pub kids: Option<TKids>,
    #[serde(default)]
    pub habits: Option<THabits>,
    pub prompts: TPrompts,
    pub images: TImages,
}

#[cfg(feature = "server")]
impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Person {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;

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
        let kids = try_opt!(TKids, "kids");
        let habits = try_opt!(THabits, "habits");

        let prompts = match row.try_get::<TPrompts, _>("prompts") {
            Ok(v) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => Json(Vec::new()),
            Err(e) => return Err(e),
        };

        let images = match row.try_get::<TImages, _>("images") {
            Ok(v) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => Json(Vec::new()),
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
            kids,
            habits,
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

    pub fn images(&self) -> &Vec<Image> {
        #[cfg(feature = "server")]
        {
            self.images.as_ref()
        }

        #[cfg(not(feature = "server"))]
        &self.images
    }

    pub fn prompts(&self) -> &Vec<Prompt> {
        #[cfg(feature = "server")]
        {
            self.prompts.0.as_ref()
        }

        #[cfg(not(feature = "server"))]
        &self.prompts
    }
}
