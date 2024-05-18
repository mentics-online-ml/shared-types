pub mod convert;

use std::time::{SystemTime, UNIX_EPOCH};
use chrono::NaiveDateTime;
use ndarray::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

// TODO: where to define this config?
pub const NUM_FEATURES:usize = 4;
pub const SERIES_LENGTH:usize = 128;

// pub const EVENT_ID_FIELD: &str = "event_id";

// This is a unique order preserving counter for the event that is used across all the data in a partition.
pub type EventId = u64;

/// Type to use for data series values.
pub type SeriesFloat = f32;

/// Type to use for model parameters.
pub type ModelFloat = f32;

/// Type to use for timestamps everywhere. Might change to a struct sometime to be a new type.
pub type Timestamp = i64;
pub type UtcDateTime = NaiveDateTime;

// It's very likely we will be running on multiple different architectures, so a central place
// to ensure binary compatibility is important. For example, t4g.small (general use) instances are ARM64, while g6.xlarge (for GPU) are X86_64.
pub fn event_id_to_bytes(e:EventId) -> [u8; 8] { e.to_le_bytes() }
pub fn float_to_bytes(f:SeriesFloat) -> [u8; 4] { f.to_le_bytes() }
pub fn bytes_to_event_id(b:[u8;8]) -> EventId { u64::from_le_bytes(b) }

/// TODO: not the best place for this, but...
pub fn now() -> Timestamp {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid system time").as_millis() as Timestamp
}

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

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub bid: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub biddate: Timestamp,
    pub ask: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub askdate: Timestamp,
}

// #[derive(Debug)]
// pub struct Raw {
//     pub id: EventId,
//     pub raw: String,
// }

#[derive(Debug)]
pub struct Event {
    pub id: EventId,
    pub x: Array1::<SeriesFloat>, // size = NUM_FEATURES
}

#[derive(Debug)]
pub struct EventSeries {
    pub id: EventId,
    pub x: Array2::<SeriesFloat>, // size = NUM_FEATURES x SERIES_LENGTH
}

#[derive(Debug,Default)]
/// The result of an inference.
pub struct Inference {
    pub value: SeriesFloat
}

#[derive(Debug)]
/// The id is of the most recent event that was included in the inference.
pub struct Inferred {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub inference: Inference,
}

#[derive(Debug,Default)]
pub struct Label {
    pub value: Array1::<SeriesFloat>, // size = NUM_FEATURES
}

#[derive(Debug,Default)]
pub struct Labelled {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub label: Label,
}