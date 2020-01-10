use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct JSONParserConfig {
    pub id: Option<i32>,
    pub metric_name: Option<String>,
    pub default_tags: Option<HashMap<String, String>>,

    pub tag_keys: Option<Vec<String>>,
    pub json_string_fields: Option<Vec<String>>,
    pub json_name_key: Option<String>,
    pub json_time_key: Option<String>,
    pub json_time_format: Option<String>,
    pub json_time_zone: Option<String>,
}
