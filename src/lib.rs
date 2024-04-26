pub mod convert;

use std::fmt::Display;
use serde_json::{self, Value};
use ndarray::prelude::*;

// TODO: where to define this config?
pub const NUM_FEATURES:usize = 4;
pub const SERIES_LENGTH:usize = 128;

pub const EVENT_ID_FIELD: &str = "event_id";

// This is a unique order preserving counter for the event that is used across all the data in a partition.
pub type EventId = u64;
pub type Float = f32;

#[derive(Debug)]
pub struct FeaturesError {
    // pub under:Box<dyn Display>
    pub under:String
}

pub struct Features {
    pub id: EventId,
    pub x: Array1::<Float>, // size = NUM_FEATURES
}

pub struct FeaturesSeries {
    pub id: EventId,
    pub x: Array2::<Float>, // size = NUM_FEATURES x SERIES_LENGTH
}

/// The result of an inference.
pub struct Inference {
}

/// The id is of the most recent event that was included in the inference.
pub struct Inferred {
    pub id: EventId,
    pub inf: Inference,
}