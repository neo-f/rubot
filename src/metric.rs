use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Local};

pub enum MetricType {
    Counter,
    Gauge,
    Untyped,
    Summary,
    Histogram,
}

pub enum Value {
    /// Represents a TOML string
    String(String),
    /// Represents a TOML integer
    Integer(i64),
    /// Represents a TOML float
    Float(f64),
    /// Represents a TOML boolean
    Boolean(bool),
}

pub struct Field {
    key: String,
    value: Value,
}

pub struct Tag {
    key: String,
    value: String,
}

pub struct Metric {
    pub name: String,
    pub tags: BTreeMap<String, String>,
    pub fields: BTreeMap<String, Value>,
    pub tm: DateTime<Local>,
    pub tp: MetricType,
    pub aggregate: bool,
}

impl Metric {
    pub fn new<S: Into<String>>(
        name: S,
        tags: BTreeMap<String, String>,
        fields: BTreeMap<String, Value>,
        tm: DateTime<Local>,
        tp: MetricType,
    ) -> Self {
        Self {
            name: name.into(),
            tags,
            fields,
            tm,
            tp,
            aggregate: false,
        }
    }

    pub fn tag_list(&self) -> Vec<Tag> {
        unimplemented!()
    }

    pub fn field_list(&self) -> Vec<Field> {
        unimplemented!()
    }

    pub fn accept(&mut self) {
        unimplemented!()
    }

    pub fn reject(&mut self) {
        unimplemented!()
    }

    pub fn drop(&mut self) {
        unimplemented!()
    }

    pub fn set_aggregate(&mut self, a: bool) {
        unimplemented!()
    }

    pub fn is_aggregate(&self) -> bool {
        unimplemented!()
    }
}

pub fn make_metric(
    mut m: Metric,
    name_override: Option<String>,
    name_prefix: Option<String>,
    name_suffix: Option<String>,
    tags: HashMap<String, String>,
    global_tags: HashMap<String, String>,
) {
    if let Some(name) = name_override {
        m.name = name
    }
    if let Some(prefix) = name_prefix {
        m.name = prefix + &m.name
    }
    if let Some(suffix) = name_suffix {
        m.name += &suffix
    }
    for (k, v) in tags {
        m.tags.insert(k, v).unwrap();
    }
    for (k, v) in global_tags {
        m.tags.insert(k, v).unwrap();
    }
}
