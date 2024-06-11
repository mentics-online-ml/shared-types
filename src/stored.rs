use crate::*;
use data_info::{InputRaw, LabelType};


pub type LabelTypeStored = LabelType; // [u8; std::mem::size_of::<LabelType>()];
// TODO: std::mem::size_of::<InputData>()
pub type InputStored = InputRaw; // [ModelFloat; 1];
pub type OutputStored = LabelTypeStored;

#[derive(Debug, Default)]
pub struct LabelStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub offset_from: OffsetId,
    pub offset_to: OffsetId,
    pub label: LabelTypeStored,
}

/// The id is of the most recent event that was included in the training.
/// However this uses information from label.offset_to (join on event_id)
pub struct TrainStored {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub offset: OffsetId,
    pub loss: LossType,
    pub input: InputStored,
    pub output: OutputStored,
}

pub struct TrainStoredWithLabel {
    pub event_id: EventId,
    pub timestamp: Timestamp,
    pub offset: OffsetId,
    pub loss: LossType,
    pub input: InputStored,
    pub output: OutputStored,
    pub label: LabelTypeStored,
}

// pub type InferType = LabelType;

// /// The result of an inference.
// #[derive(Debug,Default)]
// pub struct Infer {
//     pub value: InferType
// }

// #[derive(Debug)]
// /// The id is of the most recent event that was included in the inference.
// pub struct InferStored {
//     pub event_id: EventId,
//     pub timestamp: Timestamp,
//     pub inference: Infer,
// }
