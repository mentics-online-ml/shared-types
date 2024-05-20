pub mod util;
pub mod convert;

use chrono::{DateTime, NaiveDateTime};
use chrono_tz::Tz;
// use ndarray::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

pub type VersionType = u32;

// TODO: where to define this config?
pub const NUM_FEATURES: usize = 4;
pub const SERIES_LENGTH: usize = 1024;

// pub const EVENT_ID_FIELD: &str = "event_id";

// This is a unique order preserving counter for the event that is used across all the data in a partition.
pub type EventId = u64;

/// Type to use for data series values.
pub type SeriesFloat = f32;

/// Type to use for model parameters.
pub type ModelFloat = f32;
pub const MODEL_OUTPUT_WIDTH: usize = 8;

/// Type to use for timestamps everywhere. Might change to a struct sometime to be a new type.
pub type Timestamp = i64;
pub type UtcDateTime = NaiveDateTime;
pub type MarketTimestamp = DateTime<Tz>;
const MARKET_TIMEZONE: chrono_tz::Tz = chrono_tz::US::Eastern;

pub trait Logger {
    fn log(&self, msg: String);
}

pub struct StdoutLogger();

impl StdoutLogger {
    pub fn boxed() -> Box<StdoutLogger> {
        Box::new(StdoutLogger())
    }
}

impl Logger for StdoutLogger {
    fn log(&self, msg: String) {
        println!("{:?}", msg);
    }
}

pub trait SeriesEvent {
    fn set_event_id(&mut self, event_id:EventId);
}

#[derive(Debug, Deserialize)]
pub struct QuoteEvent {
    #[serde(default)]
    pub event_id: EventId,
    pub bid: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub biddate: Timestamp,
    pub ask: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub askdate: Timestamp,
}

// #[derive(Debug)]
// pub struct Event {
//     pub id: EventId,
//     pub x: Array1::<SeriesFloat>, // size = NUM_FEATURES
// }

// #[derive(Debug)]
// pub struct EventSeries {
//     pub id: EventId,
//     pub x: Array2::<SeriesFloat>, // size = NUM_FEATURES x SERIES_LENGTH
// }

/// The result of labelling.
#[derive(Debug,Default)]
pub struct Label {
    pub value: [ModelFloat; MODEL_OUTPUT_WIDTH]
}

#[derive(Debug,Default)]
pub struct Labelled {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub label: Label,
}

/// The result of training.
#[derive(Debug,Default)]
pub struct Train {
    pub loss: ModelFloat
}

/// The id is of the most recent event that was included in the inference.
#[derive(Debug)]
pub struct Trained {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub train: Train,
}

/// The result of an inference.
#[derive(Debug,Default)]
pub struct Inference {
    pub value: [ModelFloat; MODEL_OUTPUT_WIDTH]
}

#[derive(Debug)]
/// The id is of the most recent event that was included in the inference.
pub struct Inferred {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub inference: Inference,
}
