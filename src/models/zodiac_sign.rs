use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

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
    pub fn from_date(date: NaiveDate) -> Self {
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
