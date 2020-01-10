use std::collections::HashMap;

use crate::plugins::serializers::SerializerConfig;
use serde_derive::Deserialize;
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct OutputPlugin {
    pub name: String,
    pub tags: Option<HashMap<String, String>>,
    pub config: Option<Value>,
    pub serializer: SerializerConfig,
}
