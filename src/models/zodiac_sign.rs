use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "zodiac_sign", rename_all = "lowercase")
)]
pub enum ZodiacSign {
    #[strum(to_string = "♈ Aries")]
    Aries,

    #[strum(to_string = "♉ Taurus")]
    Taurus,

    #[strum(to_string = "♊ Gemini")]
    Gemini,

    #[strum(to_string = "♋ Cancer")]
    Cancer,

    #[strum(to_string = "♌ Leo")]
    Leo,

    #[strum(to_string = "♍ Virgo")]
    Virgo,

    #[strum(to_string = "♎ Libra")]
    Libra,

    #[strum(to_string = "♏ Scorpio")]
    Scorpio,

    #[strum(to_string = "♐ Sagittarius")]
    Sagittarius,

    #[strum(to_string = "♑ Capricorn")]
    Capricorn,

    #[strum(to_string = "♒ Aquarius")]
    Aquarius,

    #[strum(to_string = "♓ Pisces")]
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
