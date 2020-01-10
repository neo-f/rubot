use serde_derive::Deserialize;

use csv::CSVParserConfig;
use influx::InfluxParserConfig;
use json::JSONParserConfig;

pub mod csv;
pub mod influx;
pub mod json;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ParserConfig {
    #[serde(rename = "json")]
    JSON(JSONParserConfig),
    #[serde(rename = "influx")]
    Influx(InfluxParserConfig),
    #[serde(rename = "csv")]
    CSV(CSVParserConfig),
}
