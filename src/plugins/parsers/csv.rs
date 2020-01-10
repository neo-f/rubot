use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CSVParserConfig {
    pub id: Option<i32>,
    pub metric_name: Option<String>,
    pub default_tags: Option<HashMap<String, String>>,

    pub column_names: Vec<String>,
    pub column_types: Vec<String>,
    pub comment: String,
    pub delimiter: String,
    pub header_row_count: usize,
    pub measurement_column: String,
    pub skip_columns: Vec<String>,
    pub skip_rows: usize,
    pub tag_columns: Vec<String>,
    pub timestamp_column: String,
    pub timestamp_format: String,
    pub trim_space: bool,
}
