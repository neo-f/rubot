use crate::agent::MetricMaker;
use crate::config;
use crate::error::AppError;
use crate::metric::{make_metric, Metric};
use std::time;

pub trait Input {
    fn gather() -> Result<(), AppError>;
}

pub struct RunningInput {
    pub config: config::InputPlugin,
}

impl MetricMaker for RunningInput {
    fn make_metric(&self, m: Metric) -> Metric {
        unimplemented!()
    }
}

pub struct RunningOutput {
    pub config: config::OutputPlugin,
}

pub struct RunningConfig {
    pub inputs: Vec<RunningInput>,
    pub outputs: Vec<RunningOutput>,
    pub precision: u16,
    pub interval: time::Duration
}
