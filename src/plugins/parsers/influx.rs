use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct InfluxParserConfig {
    pub id: Option<i32>,
    pub metric_name: Option<String>,
    pub default_tags: Option<HashMap<String, String>>,
}
