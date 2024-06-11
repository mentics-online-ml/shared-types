use chrono_util::{ChronoFeatures, CHRONO_BYTE_SIZE};

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
pub const TIME_ENCODING_SIZE: usize = 4;

pub const FEATURES1_SIZE: usize = 2;
pub const SERIES1_ITEM_SIZE: usize = TIME_ENCODING_SIZE + FEATURES1_SIZE;

pub type TimeEncoding = [ModelFloat; TIME_ENCODING_SIZE];
pub type FeatureEncoding = [ModelFloat; FEATURES1_SIZE];
pub type SeriesItem = [ModelFloat; TIME_ENCODING_SIZE + FEATURES1_SIZE];
pub type Series = [SeriesItem; SERIES1_SIZE];
pub const SERIES_BYTE_SIZE: usize = std::mem::size_of::<Series>();

pub type InputRaw = (ChronoFeatures, Series);
pub const INPUT_RAW_BYTE_SIZE: usize = CHRONO_BYTE_SIZE + SERIES_BYTE_SIZE;
pub type InputRawBatch<const N: usize> = (BatchOf<ChronoFeatures,N>, BatchOf<Series,N>);
// pub type OutputRaw = LabelType;
// pub type OutputRawBatch<const N: usize> = BatchOf<LabelType,N>;


pub type LabelType = [ModelFloat; MODEL_OUTPUT_WIDTH];
