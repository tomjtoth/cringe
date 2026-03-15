use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Children {
    pub has: Option<i8>,
    pub wants: Option<i8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Pic {
    Url(String),
    Advanced { url: String, prompt: Option<String> },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Liked {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Frequency {
    Never,
    Rarely,
    Often,
    #[serde(rename = "yes")]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonPrompt {
    pub title: String,
    pub body: String,
}

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub city: Option<String>,
    pub lat: f64,
    #[serde(alias = "long")]
    pub lon: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub gender: Gender,
    pub born: NaiveDate,
    pub height: u8,
    pub occupation: Option<String>,
    pub location: Location,
    pub children: Option<Children>,
    pub drinking: Option<Frequency>,
    pub smoking: Option<Frequency>,
    pub marijuana: Option<Frequency>,
    pub drugs: Option<Frequency>,
    pub prompts: Vec<PersonPrompt>,
    pub pictures: Vec<Pic>,
    pub liked: Option<Liked>,
}

impl Person {
    pub fn age(&self) -> u32 {
        let today = Utc::now().date_naive();
        today.years_since(self.born).unwrap()
    }

    pub fn zodiac_sign(&self) -> ZodiacSign {
        ZodiacSign::from_date(self.born)
    }
}
