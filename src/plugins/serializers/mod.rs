use serde_derive::Deserialize;
use std::time;
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SerializerConfig {
    #[serde(rename = "json")]
    JSON(JSONSerializerConfig),
    #[serde(rename = "influx")]
    Influx(InfluxSerializerConfig),
    #[serde(rename = "vue_table")]
    VueTable,
}

#[derive(Deserialize, Debug)]
pub struct JSONSerializerConfig {
    timestamp_units: time::Duration,
}

#[derive(Deserialize, Debug)]
pub struct InfluxSerializerConfig {
    #[serde(default)]
    sort_fields: bool,
    #[serde(default)]
    uint_support: bool,
    #[serde(default)]
    max_line_bytes: usize,
}
