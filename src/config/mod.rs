use std::{fs, time};

use serde_derive::Deserialize;

pub use input::InputPlugin;
pub use output::OutputPlugin;

use crate::error::AppError;
use crate::model::RunningConfig;

pub mod input;
pub mod output;

const CONFIG_PATH: &'static str = "config.toml";

#[derive(Deserialize, Debug)]
pub struct AgentConfig{
    // RoundInterval 将收集间隔四舍五入至具体的'interval值'
    // ie, Interval=10s then always collect on :00, :10, :20, etc.
    pub round_interval: bool,

    // Interval 数据收集间隔
    #[serde(with = "serde_humanize_rs")]
    pub interval: time::Duration,

    // flush_buffer_when_full  当output缓冲区满了之后是否执行一次写入
    // 如果开启此项，当缓冲区满了之后，会忽略flush_interval，立即执行一次写入操作
    pub flush_buffer_when_full: bool,

    // 时间精度, 秒后小数点个数
    // ie, precision(0) => 精确到秒 s
    //     precision(3) => 精确到毫秒 ms
    pub precision: u16,

    // collection_jitter 数据收集抖动间隔
    // 在开启了很多input插件的情况下，如果同一时间同时收集，可能会对系统造成较高的负载(如sysfs)
    // 开启此项后在进行数据采集时，会随机休眠 jitter 范围内的一段时间再进行采集任务，可以缓解该情况
    // ie, collection_jitter = 100ms, interval = 10s 意味着每 9s900ms-10s100ms进行一次收集
    #[serde(with = "serde_humanize_rs")]
    pub collection_jitter: time::Duration,

    // flush_interval 数据写入output插件的间隔
    #[serde(with = "serde_humanize_rs")]
    pub flush_interval: time::Duration,

    // flush_jitter 数据写入抖动间隔
    // 参照collection_jitter
    #[serde(with = "serde_humanize_rs")]
    pub flush_jitter : time::Duration,

    // metric_batch_size 每次单独写入一个output插件的最大metric数量
    pub metric_batch_size: usize,

    // metric_batch_size 每个output插件缓冲区的大小
    // 当有一次成功写入时，缓存将被清空，当缓冲区满了之后，旧的数据将会被替换掉
    // 缓冲区大小一般不小于两倍的metric_batch_size
    pub metric_buffer_limit: usize,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub agent: AgentConfig,
    pub input_plugin: Vec<InputPlugin>,
    pub output_plugin: Vec<OutputPlugin>,
}

impl Config {
    pub fn new() -> Result<Config, AppError> {
        let content = fs::read_to_string(CONFIG_PATH)?;
        let config = toml::from_str::<Config>(&content)?;
        Ok(config)
    }
}
