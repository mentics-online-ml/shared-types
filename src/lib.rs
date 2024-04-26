pub mod convert;

use std::fmt::Display;
use serde_json::{self, Value};
use ndarray::prelude::*;

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

// It's very likely we will be running on multiple different architectures, so a central place
// to ensure binary compatibility is important. For example, t4g.small (general use) instances are ARM64, while g6.xlarge (for GPU) are X86_64.
pub fn event_id_to_bytes(e:EventId) -> [u8; 8] { e.to_le_bytes() }
pub fn float_to_bytes(f:SeriesFloat) -> [u8; 4] { f.to_le_bytes() }
pub fn bytes_to_event_id(b:[u8;8]) -> EventId { u64::from_le_bytes(b) }

#[derive(Debug)]
pub struct FeaturesError {
    // pub under:Box<dyn Display>
    pub under:String
}

#[derive(Debug)]
pub struct Raw {
    pub id: EventId,
    pub raw: String,
}

#[derive(Debug)]
pub struct Features {
    pub id: EventId,
    pub x: Array1::<SeriesFloat>, // size = NUM_FEATURES
}

#[derive(Debug)]
pub struct FeaturesSeries {
    pub id: EventId,
    pub x: Array2::<SeriesFloat>, // size = NUM_FEATURES x SERIES_LENGTH
}

#[derive(Debug)]
/// The result of an inference.
pub struct Inference {
}

#[derive(Debug)]
/// The id is of the most recent event that was included in the inference.
pub struct Inferred {
    pub id: EventId,
    pub inf: Inference,
}