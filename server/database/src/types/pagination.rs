use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};
use tracing::debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    #[serde(default = "default_page", deserialize_with = "default_page_if_empty")]
    pub page: i64,
    #[serde(
        default = "default_page_size",
        deserialize_with = "default_page_size_if_empty"
    )]
    pub size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

fn default_page_if_empty<'de, D>(de: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    debug!("{:#?}", opt);
    match opt.as_deref() {
        None | Some("") => Ok(default_page()),
        Some(s) => i64::from_str(s).map_err(de::Error::custom),
    }
}
fn default_page_size_if_empty<'de, D>(de: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    debug!("{:#?}", opt);
    match opt.as_deref() {
        None | Some("") => Ok(default_page_size()),
        Some(s) => i64::from_str(s).map_err(de::Error::custom),
    }
}
