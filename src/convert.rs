use std::mem::MaybeUninit;

use crate::*;

// It's very likely we will be running on multiple different architectures, so a central place
// to ensure binary compatibility is important. For example, t4g.small (general use) instances are ARM64, while g6.xlarge (for GPU) are X86_64.
pub fn event_id_to_bytes(e:EventId) -> [u8; 8] { e.to_le_bytes() }
pub fn float_to_bytes(f:SeriesFloat) -> [u8; 4] { f.to_le_bytes() }
pub fn bytes_to_event_id(b:[u8;8]) -> EventId { EventId::from_le_bytes(b) }

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
    unsafe {
        std::mem::transmute(x)
    }
}

pub fn to_model_input_flat(x: ModelInput) -> ModelInputFlat {
    unsafe {
        std::mem::transmute(x)
    }
}

pub fn input_to_bytes(input: ModelInput) -> Vec<u8> {
    input.into_iter().flat_map(
        |arr| arr.into_iter().map(|x| x.to_ne_bytes())
    ).flatten().collect()
}

pub fn input_from_bytes(bytes: &[u8]) -> ModelInput {
    let mut input: [[MaybeUninit<ModelFloat>; NUM_FEATURES]; SERIES_SIZE] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    loop {
        // let x: &f32 = bytemuck::from_bytes(&bytes[i..i+4]);
        let four_bytes: &[u8; 4] = bytemuck::from_bytes(&bytes[i..i+4]);
        let x: f32 = f32::from_ne_bytes(*four_bytes);
        input[k][j].write(x);
        j += 1;
        if j == NUM_FEATURES {
            j = 0;
            k += 1;
            if k == SERIES_SIZE {
                break;
            }
        }
        i += 4;
    }
    unsafe { std::mem::transmute::<_, ModelInput>(input) }

    // bytes.chunks_exact(4)
    //     .map(TryInto::try_into)
    //     .map(Result::unwrap)
    //     .map(f32::from_ne_bytes)
    //     .array_chunks::<6>()
    //     .collect::<Vec<ModelInputInner>>().try_into().unwrap()
}

pub fn output_to_bytes(input: ModelOutput) -> Vec<u8> {
    input.into_iter().flat_map(
        |x| x.to_ne_bytes()
    ).collect()
}

pub fn output_from_bytes(bytes: &[u8]) -> ModelOutput {
    let v: Vec<f32> = bytes.chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(f32::from_ne_bytes)
        .collect();
    v.try_into().unwrap()
}
