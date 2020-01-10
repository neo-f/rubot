use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::metric;
use crate::metric::Metric;

pub trait TAccumulator {
    // AddFields adds a metric to the accumulator with the given measurement
    // name, fields, and tags (and timestamp).
    fn add_fields<S: Into<String>>(
        &mut self,
        measurement: S,
        fields: HashMap<String, metric::Value>,
        tags: HashMap<String, String>,
        time: DateTime<Local>,
    );

    // AddMetric adds an metric to the accumulator.
    fn add_metric<'a>(&mut self, m: Metric);
}
