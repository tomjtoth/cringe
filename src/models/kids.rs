use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Kids {
    pub has: Option<i8>,
    pub wants: Option<i8>,
}

#[cfg(feature = "server")]
pub type TKids = sqlx::types::Json<Kids>;

#[cfg(not(feature = "server"))]
pub type TKids = Kids;
