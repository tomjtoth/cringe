use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::types::Json;

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

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kids {
    pub has: Option<i8>,
    pub wants: Option<i8>,
}

#[cfg(feature = "server")]
type TKids = Json<Kids>;

#[cfg(not(feature = "server"))]
type TKids = Kids;

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pic {
    Url(String),
    Advanced {
        url: String,
        prompt: Option<String>,
    },
    Uploaded {
        bytes: Vec<u8>,
        mime_type: String,
        prompt: Option<String>,
    },
}

impl Pic {
    pub fn prompt(&self) -> Option<&str> {
        match self {
            Self::Url(_) => None,
            Self::Advanced { prompt, .. } | Self::Uploaded { prompt, .. } => prompt.as_deref(),
        }
    }

    pub fn src(&self) -> String {
        match self {
            Self::Url(src) => src.clone(),
            Self::Advanced { url, .. } => url.clone(),
            Self::Uploaded {
                bytes, mime_type, ..
            } => {
                use base64::{engine::general_purpose::STANDARD, Engine as _};

                format!("data:{mime_type};base64,{}", STANDARD.encode(bytes))
            }
        }
    }
}

#[cfg(feature = "server")]
type TPics = Json<Vec<Pic>>;
#[cfg(not(feature = "server"))]
type TPics = Vec<Pic>;


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

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Never => "Never",
            Self::Rarely => "Rarely",
            Self::Often => "Often",
            Self::YesPlease => "Yes, please!",
        };

        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "seeking"))]
pub enum Seeking {
    #[serde(rename = "short-term fun")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term fun"))]
    ShortTermFun,
    #[serde(rename = "short-term, open to long")]
    #[cfg_attr(feature = "server", sqlx(rename = "short-term, open to long"))]
    ShortTermOpenToLong,
    #[serde(rename = "long-term, open to short")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term, open to short"))]
    LongTermOpenToShort,
    #[serde(rename = "long-term")]
    #[cfg_attr(feature = "server", sqlx(rename = "long-term"))]
    LongTerm,
    #[serde(rename = "still figuring it out")]
    #[cfg_attr(feature = "server", sqlx(rename = "still figuring it out"))]
    StillFiguringItOut,
}

impl std::fmt::Display for Seeking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::ShortTermFun => "🎉 Short-term fun",
            Self::ShortTermOpenToLong => "🪄 Short-term, open to long",
            Self::LongTermOpenToShort => "🍷 Long-term, open to short",
            Self::LongTerm => "❤️ Long-term",
            Self::StillFiguringItOut => "🤔 Still figuring it out",
        };

        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Monogamy => "💍 Monogamy",
            Self::NonMonogamy => "🌈 Non-monogamy",
            Self::FiguringOutMyRelationshipType => "🧭 Figuring out my relationship type",
        };

        f.write_str(label)
    }
}

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonPrompt {
    pub title: String,
    pub body: String,
}

#[cfg(feature = "server")]
type TPrompts = Json<Vec<PersonPrompt>>;
#[cfg(not(feature = "server"))]
type TPrompts = Vec<PersonPrompt>;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    pub born: NaiveDate,

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
    pub distance: Option<i16>,

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
    pub pictures: TPics,
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
        let born: NaiveDate = row.try_get("born")?;
        let height: i16 = row.try_get("height")?;

        let education = try_opt!(String, "education");
        let occupation = try_opt!(String, "occupation");

        let location = try_opt!(String, "location");

        let gps = match row.try_get::<Option<Gps>, _>("gps") {
            Ok(v) => v,
            Err(sqlx::Error::ColumnNotFound(_)) => None,
            Err(e) => return Err(e),
        };

        let distance = try_opt!(i16, "distance");

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

        let pictures = match row.try_get::<TPics, _>("pictures") {
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
            pictures,
        })
    }
}

impl Person {
    pub fn age(&self) -> u32 {
        let today = Utc::now().date_naive();
        today.years_since(self.born).unwrap()
    }

    pub fn pics(&self) -> &Vec<Pic> {
        #[cfg(feature = "server")]
        {
            self.pictures.as_ref()
        }

        #[cfg(not(feature = "server"))]
        &self.pictures
    }

    pub fn prompts(&self) -> &Vec<PersonPrompt> {
        #[cfg(feature = "server")]
        {
            self.prompts.as_ref()
        }

        #[cfg(not(feature = "server"))]
        &self.prompts
    }

    pub fn zodiac_sign(&self) -> ZodiacSign {
        ZodiacSign::from_date(self.born)
    }
}
