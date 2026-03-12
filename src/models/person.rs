use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub gender: Gender,
    pub born: NaiveDate,
    pub height: u8,
    pub occupation: Option<String>,
    pub children: Option<Children>,
    pub prompts: HashMap<String, String>,
    pub pictures: Vec<Pic>,
    pub liked: Option<Liked>,
}

impl Person {
    pub fn age(&self) -> u32 {
        let today = Utc::now().date_naive();
        today.years_since(self.born).unwrap()
    }
}
