#![feature(trait_alias)]

pub mod util;
pub mod convert;
mod implement;
pub mod series_proc;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use chrono_tz::Tz;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_aux::prelude::*;
use util::{same_date, to_market_datetime, ts_in_trading_time};

pub type VersionType = u32;
/// CURRENT_VERSION should only be used in main.rs files so that all other objects receive it.
pub const CURRENT_VERSION: VersionType = 1;

pub const TIME_ENCODING_WIDTH: usize = 4;
pub const NUM_FEATURES: usize = 2 + TIME_ENCODING_WIDTH;
pub const SERIES_SIZE: usize = 1024;
pub const SERIES_LENGTH: OffsetId = SERIES_SIZE as OffsetId;

// This is a unique order preserving counter for the event that is used across all the data in a partition.
pub type EventId = u64;

pub type OffsetId = i64;
pub type PartitionId = i32;

/// Type to use for data series values.
pub type SeriesFloat = f32;

/// Type to use for model parameters.
pub type ModelFloat = f32;
pub const MODEL_OUTPUT_WIDTH: usize = 8;
pub type ModelInput = [[ModelFloat; NUM_FEATURES]; SERIES_SIZE];
pub type ModelInputFlat = [ModelFloat; SERIES_SIZE * NUM_FEATURES];

pub fn new_input() -> ModelInput {
    [[0f32; NUM_FEATURES]; SERIES_SIZE]
}

/// Type to use for timestamps everywhere. Might change to a struct sometime to be a new type.
pub type Timestamp = i64;
pub type UtcDateTime = NaiveDateTime;
pub type MarketTimestamp = DateTime<Tz>;
const MARKET_TIMEZONE: chrono_tz::Tz = chrono_tz::US::Eastern;
const INVALID_DATE: NaiveDate = NaiveDate::MIN;

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

pub enum Validity {
    Valid,
    CauseReset,
    Invalid,
}

pub trait SeriesEvent {
    fn set_ids(&mut self, event_id: EventId, offset: OffsetId);
    fn timestamp(&self) -> Timestamp;

    // TODO: base should be generic, but... lazy for now
    fn validity(&self, base: &QuoteValues) -> Validity {
        if !self.event_in_trading_time() {
            Validity::Invalid
        } else if !same_date(self.to_date_or_0(), base.date_or_0) {
            Validity::CauseReset
        } else {
            Validity::Valid
        }
    }

    fn event_in_trading_time(&self) -> bool {
        ts_in_trading_time(self.timestamp())
    }

    fn to_date_or_0(&self) -> NaiveDate {
        to_market_datetime(self.timestamp()).date_naive()
    }
}

pub trait EventType = SeriesEvent + DeserializeOwned;

pub trait EventHandler<T: EventType> {
    fn handle(&mut self, event: T) -> bool;
}

// pub trait EventProcessor<T: EventType> {
//     fn handle(&mut self, event: &T) -> bool;
// }

pub trait Processor<T,S> {
    fn process(&mut self, start_values: &S, x: &mut T) -> bool;
    fn reset(&mut self) {
        // default do nothing
    }
}

// #[derive(Default)]
// pub struct NoopProcessor<T>(T);

// impl<T> Processor<T> for NoopProcessor<T> {
//     fn process(&mut self, _: &T) -> bool {
//         false
//     }
// }

pub trait BaseValues<T> {
    fn convert_from(event: &T) -> Self;
    fn validity(&self, event: &T) -> Validity;
}

///
/*
{
"type":"quote"
"symbol":"SPX"
"bid":5249.61
"bidsz":0
"bidexch":""
"biddate":"1715716641000"
"ask":5250.74
"asksz":0
"askexch":""
"askdate":"1715716641000"
}
*/
///
// pub struct InputQuote {
//     pub bid: f32,
//     pub bid_size: f32,
//     pub bid_ts: u64,
//     pub ask: f32,
//     pub ask_size: f32,
//     pub ask_ts: u64,
// }


/// Published to series by ingest and read by label, train...
#[derive(Debug, Deserialize)]
pub struct QuoteEvent {
    #[serde(default)]
    pub event_id: EventId,
    #[serde(default)]
    pub offset: OffsetId,
    pub bid: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub biddate: Timestamp,
    pub ask: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub askdate: Timestamp,
}

// impl Clone for QuoteEvent {
//     fn clone(&self) -> Self {
//         Self { event_id: self.event_id.clone(), offset: self.offset.clone(), bid: self.bid.clone(), biddate: self.biddate.clone(), ask: self.ask.clone(), askdate: self.askdate.clone() }
//     }
// }

// impl Copy for QuoteEvent {

// }

// impl TryFrom<&QuoteEvent> for [f32; NUM_FEATURES] {
//     type Error = ();

//     fn try_from(q: &QuoteEvent) -> Result<Self, Self::Error> {
//         Ok([q.bid, q.ask])
//     }
// }

#[derive(Default)]
pub struct QuoteValues {
    pub date_or_0: NaiveDate,
    pub bid: SeriesFloat,
    pub ask: SeriesFloat
}

// impl Default for QuoteValues {
//     fn default() -> Self {
//         Self { date_or_0: NaiveDate::default(), bid: 0.0, ask: 0.0 }
//     }
// }

// impl From<&QuoteEvent> for QuoteValues {
//     fn from(event: &QuoteEvent) -> Self {
//         Self { bid: event.bid, ask: event.ask }
//     }
// }

impl BaseValues<QuoteEvent> for QuoteValues {
    fn convert_from(event: &QuoteEvent) -> Self {
        Self { date_or_0: event.to_date_or_0(), bid: event.bid, ask: event.ask }
    }

    fn validity(&self, event: &QuoteEvent) -> Validity {
        if !event.event_in_trading_time() {
            Validity::Invalid
        } else if !same_date(event.to_date_or_0(), self.date_or_0) {
            Validity::CauseReset
        } else {
            Validity::Valid
        }
    }
}


pub type LabelType = [ModelFloat; MODEL_OUTPUT_WIDTH];
/// The result of labelling.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Label {
    pub value: LabelType
}

/// Published to series by label and read by train.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LabelEvent {
    pub event_id: EventId,
    pub offset_from: OffsetId,
    pub offset_to: OffsetId,
    pub timestamp: Timestamp,
    pub label: Label
}
impl LabelEvent {
    pub fn new(event_id: EventId, timestamp: Timestamp, offset_from: OffsetId, offset_to: OffsetId, label: Label) -> Self {
        Self { event_id, offset_from, offset_to, timestamp, label }
    }
}

#[derive(Debug, Default)]
pub struct LabelStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub partition: PartitionId,
    pub offset_from: OffsetId,
    pub offset_to: OffsetId,
    pub label: Label,
}

pub type TrainResultType = ModelFloat;
pub type BatchTrainResultType = Vec<ModelFloat>;
pub type ModelOutput = LabelType;

/// The result of training.
#[derive(Debug,Default)]
pub struct Train {
    pub loss: TrainResultType
}

/// The id is of the most recent event that was included in the inference.
#[derive(Debug)]
pub struct TrainStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub partition: PartitionId,
    pub offset: OffsetId,
    pub input: ModelInput,
    pub label: Label,
}

pub struct TrainLossStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub loss: ModelFloat,
}

pub type InferType = [ModelFloat; MODEL_OUTPUT_WIDTH];

/// The result of an inference.
#[derive(Debug,Default)]
pub struct Infer {
    pub value: InferType
}

#[derive(Debug)]
/// The id is of the most recent event that was included in the inference.
pub struct InferStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub inference: Infer,
}
