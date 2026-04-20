use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Kids {
    pub has: Option<i8>,
    pub wants: Option<i8>,
}
