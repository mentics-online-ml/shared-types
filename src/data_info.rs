use chrono_util::{ChronoFeatures, CHRONO_BYTE_SIZE};
use serde_json::json;

use crate::*;

pub type BatchOf<T, const N: usize> = [T; N];

/// Model static params: begin ///
pub const MODEL_OUTPUT_WIDTH: usize = 8;

/// CURRENT_VERSION should only be used in main.rs files so that all other objects receive it.
pub const CURRENT_VERSION: VersionType = 1;
/// Model static params: end ///


pub const SERIES1_FEATURES_SIZE: usize = 2;
pub const SERIES1_SIZE: usize = 1024;
pub const SERIES1_LENGTH: OffsetId = SERIES1_SIZE as OffsetId;

pub const FEATURES1_SIZE: usize = 2;
pub const SERIES1_ITEM_SIZE: usize = TIME_EMBEDDING_SIZE + FEATURES1_SIZE;

pub type TimeEncoding = [ModelFloat; TIME_EMBEDDING_SIZE];
pub type FeatureEncoding = [ModelFloat; FEATURES1_SIZE];
pub type SeriesItem = [ModelFloat; TIME_EMBEDDING_SIZE + FEATURES1_SIZE];
pub type Series = [SeriesItem; SERIES1_SIZE];
pub const SERIES_BYTE_SIZE: usize = std::mem::size_of::<Series>();

pub type InputRaw = (ChronoFeatures, Series);
pub const INPUT_RAW_BYTE_SIZE: usize = CHRONO_BYTE_SIZE + SERIES_BYTE_SIZE;
pub type InputRawBatch<const N: usize> = (BatchOf<ChronoFeatures,N>, BatchOf<Series,N>);
// pub type OutputRaw = LabelType;
// pub type OutputRawBatch<const N: usize> = BatchOf<LabelType,N>;


pub type LabelType = [ModelFloat; MODEL_OUTPUT_WIDTH];


// ---- Common Values ---- //

pub const TIME_EMBEDDING_SIZE: usize = 4;


// ---- Config ---- //

pub fn data_config() -> DataConfig{
    let value = json!({
        "quote_streams": [
            {
                "topic_name": "raw-SPY-quote",
                "feature_size": 4,
                "time_embedding_size": TIME_EMBEDDING_SIZE
            }
        ],
        "trade_streams": [
            {
                "topic_name": "raw-SPY-trade",
                "feature_size": 4,
                "time_embedding_size": TIME_EMBEDDING_SIZE
            }
        ]
    });
    serde_json::from_value(value).unwrap()
}

// ---- Types ---- //

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataConfig {
    pub quote_streams: Vec<QuoteStreamSpec>,
    pub trade_streams: Vec<TradeStreamSpec>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct QuoteStreamSpec {
    pub topic_name: String,
    pub feature_size: usize,
    pub time_embedding_size: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TradeStreamSpec {
    pub topic_name: String,
    pub feature_size: usize,
    pub time_embedding_size: usize,
}

pub trait StreamSpec {
    fn item_size(&self) -> usize;
    //  {
    //     self.feature_size + self.time_embedding_size
    // }
}
// impl StreamSpec {
// }