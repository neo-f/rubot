use std::collections::HashMap;
use std::time;

use serde_derive::Deserialize;
use toml::Value;

use crate::plugins::parsers::ParserConfig;

#[derive(Deserialize, Debug)]
pub struct InputPlugin {
    pub name: String,
    pub tags: Option<HashMap<String, String>>,
    pub config: Option<Value>,
    #[serde(with = "serde_humanize_rs")]
    pub interval: Option<time::Duration>,
    pub parser: Option<ParserConfig>,

    pub name_override: Option<String>,
    pub measurement_prefix: Option<String>,
    pub measurement_suffix: Option<String>,
}
