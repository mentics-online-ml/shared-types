#![feature(trait_alias)]
#![feature(iter_array_chunks)]

pub mod util;
pub mod series;
pub mod convert;
pub mod series_proc;
pub mod paths;
pub mod chrono_util;
pub mod stored;
pub mod quote;
pub mod label;
pub mod data_info;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use chrono_tz::Tz;
use serde::de::DeserializeOwned;
use serde_aux::field_attributes::deserialize_number_from_string;

pub type VersionType = u32;

// This is a unique order preserving counter for the event that is used across all the data in a partition.
pub type EventId = i64;

pub type OffsetId = i64;

/// Type to use for data series values when ingesting.
pub type SeriesFloat = f32;

/// Type to use for model parameters.
pub type ModelFloat = f32;
pub type LossType = ModelFloat;
pub const MODEL_FLOAT_SIZE: usize = std::mem::size_of::<ModelFloat>();

/// Type to use for timestamps everywhere. Might change to a struct sometime to be a new type.
pub type Timestamp = i64;
pub type UtcDateTime = NaiveDateTime;
pub type MarketTimestamp = DateTime<Tz>;
