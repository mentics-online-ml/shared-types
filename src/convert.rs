use crate::*;

// It's very likely we will be running on multiple different architectures, so a central place
// to ensure binary compatibility is important. For example, t4g.small (general use) instances are ARM64, while g6.xlarge (for GPU) are X86_64.
pub fn event_id_to_bytes(e:EventId) -> [u8; 8] { e.to_le_bytes() }
pub fn float_to_bytes(f:SeriesFloat) -> [u8; 4] { f.to_le_bytes() }
pub fn bytes_to_event_id(b:[u8;8]) -> EventId { u64::from_le_bytes(b) }

pub fn serialize_timestamp(timestamp: &UtcDateTime) -> Timestamp {
    // We don't need to convert to UTC first as the warning says because we always have it UTC for NaiveDateTime.
    #[allow(deprecated)]
    timestamp.timestamp_millis()
}

pub fn vec_flat_to_input(v: Vec<ModelFloat>) -> anyhow::Result<ModelInput> {
    // println!("vec_flat_to_input, len: {}", v.len());
    let y = TryInto::<ModelInputFlat>::try_into(v);
    let x = y.map_err(|e| anyhow::anyhow!("Error converting to ModelFlatInput: {:?}", e))?;
    // .with_context("Tried to convert vec to ModelInputFlat")?;
    Ok(to_model_input(x))
}

pub fn vec_flat_to_label(v: Vec<ModelFloat>) -> anyhow::Result<LabelType> {
    TryInto::<LabelType>::try_into(v).map_err(|e| anyhow::anyhow!("{:?}", e))
}

pub fn to_model_input(x: ModelInputFlat) -> ModelInput {
    // bytemuck::cast(x)
    // println!("size of array: {:?}", x.len());
    unsafe {
        std::mem::transmute(x)
    }
}

pub fn to_model_input_flat(x: ModelInput) -> ModelInputFlat {
    // x.as_flattened()
    // bytemuck::cast(x)
    unsafe {
        std::mem::transmute(x)
    }
}
