use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::ops::Deref;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread;

use chrono::{DateTime, Local, SubsecRound};

use crate::internal::TAccumulator;
use crate::metric::{Metric, MetricType, Value};
use crate::model::RunningConfig;

pub trait MetricMaker {
    fn make_metric(&self, m: Metric) -> Metric;
}

pub struct Accumulator {
    maker: Box<dyn MetricMaker>,
    sender: SyncSender<Metric>,
    precision: u16,
}

impl Accumulator {
    fn new(maker: impl MetricMaker + 'static, sender: SyncSender<Metric>, precision: u16) -> Self {
        Accumulator {
            maker: Box::new(maker),
            sender,
            precision,
        }
    }
}

impl TAccumulator for Accumulator {
    fn add_fields<S: Into<String>>(
        &mut self,
        measurement: S,
        fields: HashMap<String, Value>,
        tags: HashMap<String, String>,
        time: DateTime<Local>,
    ) {
        let fields = BTreeMap::from_iter(fields);
        let tags = BTreeMap::from_iter(tags);
        let tm = time.round_subsecs(self.precision);

        let metric = Metric::new(measurement.into(), tags, fields, tm, MetricType::Untyped);
        self.sender.send(metric).unwrap();
    }

    fn add_metric<'m>(&mut self, mut m: Metric) {
        m.tm = m.tm.round_subsecs(self.precision);
        self.sender.send(m).unwrap();
    }
}

pub struct Agent {
    config: RunningConfig,
}

impl Agent {
    pub fn new(config: RunningConfig) -> Self {
        Agent { config }
    }
    pub fn run(mut self) {}
    pub fn run_input(self, sender: SyncSender<Metric>)
    {
        let (input_sender, input_receiver) = mpsc::sync_channel::<Metric>(100);
        let input_thread = thread::spawn(move|| {
            for input in self.config.inputs {
                let acc = Accumulator::new(input, input_sender.clone(), self.config.precision);
            }
        });
    }
}
