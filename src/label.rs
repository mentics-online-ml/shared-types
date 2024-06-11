use crate::*;
use data_info::*;

// impl From<Vec<u8>> for Label {
//     fn from(bytes: Vec<u8>) -> Self {
//         Label { value: output_from_bytes(&bytes) }
//     }
// }

/// Published to series by label and read by train.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct LabelEvent {
    pub event_id: EventId,
    pub offset_from: OffsetId,
    pub offset_to: OffsetId,
    pub timestamp: Timestamp,
    pub label: LabelType
}
impl LabelEvent {
    pub fn new(event_id: EventId, timestamp: Timestamp, offset_from: OffsetId, offset_to: OffsetId, label: LabelType) -> Self {
        Self { event_id, offset_from, offset_to, timestamp, label }
    }
}
