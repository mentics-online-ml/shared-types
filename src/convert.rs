use std::{collections::VecDeque, iter::zip};

use crate::*;
use data_info::*;
use chrono_util::make_chrono_features;
use quote::QuoteEvent;
use series::SeriesEvent;

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


// TODO: use maybeuninit?
fn new_series() -> Series {
    [SeriesItem::default(); SERIES1_SIZE]
    // [[ModelFloat::default(); TIME_ENCODING_SIZE + FEATURES1_SIZE]; SERIES1_SIZE]
}

pub fn series_to_input(events: &VecDeque<QuoteEvent>) -> anyhow::Result<InputRaw> {
    assert!(events.len() == SERIES1_SIZE); // this is also checked before call above
    let mut input = new_series();

    let embedder = TimeEmbedder::<TIME_ENCODING_SIZE>::new();

    // indexing ok due to previous checking
    let base_event = &events[SERIES1_SIZE - 1];
    let QuoteEvent { bid: base_bid, ask: base_ask, .. } = base_event;
    let base_time = base_event.timestamp();

    for (event, input_column) in zip(events, input.iter_mut()) {
        let embedded_time = embedder.embed(base_time - event.timestamp());
        let bid = adjust(base_bid / event.bid);
        let ask = adjust(base_ask / event.ask);
        input_column[0] = bid;
        input_column[1] = ask;
        // std::slice::bytes::copy_memory
        // TODO: this can be optimized
        input_column[2..(2 + TIME_ENCODING_SIZE)].copy_from_slice(&embedded_time);
        // (*input_column)[0..1] = [bid, ask];
        // (*input_column)[2..(2 + TIME_ENCODING_WIDTH)] = embedded_time;
    }

    // TODO: remove this assert after test
    let mut zero_count = 0;
    for outer in input {
        for inner in outer {
            if inner == 0f32 {
                zero_count += 1;
            }
            // assert!(inner > 0.0);
        }
    }
    if zero_count > 2*events.len() {
        println!("input: {:?}", input);
    }
    assert!(zero_count < 2*events.len());

    Ok((make_chrono_features(base_time), input))
}

const MAX_TIME_SCALE: ModelFloat = 60f32 * 60f32 * 1000f32; // 1 hour in milliseconds

pub struct TimeEmbedder<const D: usize> {
    // width: usize,
    log_timescale_increment: ModelFloat,
}

// const LOG_TIMESCALE_INCREMENT: ModelFloat = -(MAX_TIME_SCALE).ln() / TIME_ENCODING_SIZE as ModelFloat;
// const TEST: TimeEmbedder<TIME_ENCODING_SIZE> = TimeEmbedder::new();

impl<const D: usize> TimeEmbedder<D> {
    pub fn new() -> Self {
        let log_timescale_increment = -(MAX_TIME_SCALE).ln() / D as ModelFloat;
        Self { log_timescale_increment }
    }

    // Adapted from burn::nn::pos_encodings::generate_sinusoids
    // times should be relative from 0 up, most recent event first
    // Returns vec of length: times.len() x width
    pub fn embed(&self, time: i64) -> [f32; D] {
        let mut result = [0f32; D];
        let time_model = time as ModelFloat;
        // let mut row = Vec::with_capacity(self.width);
        for k in (0..D).step_by(2) {
            let div_term = (k as ModelFloat * self.log_timescale_increment).exp();
            result[k] = (div_term * time_model).sin();
            result[k+1] = (div_term * time_model).cos();
        }
        result
    }
}

// fn time_embed(times: SeriesFloat, width: usize) -> Vec<ModelFloat> {
//     let d_model = width;

//     // Calculate the increment for the logarithmic timescale
//     let log_timescale_increment = -(MAX_TIME_SCALE).ln() / d_model as ModelFloat;

//     // Create a vector to hold the sinusoids for this position
//     let mut row = Vec::with_capacity(d_model / 2);
//     // Loop over each dimension of the sinusoids
//     for k in (0..d_model).step_by(2) {
//         // Calculate the division term for this dimension
//         let div_term = (k as ModelFloat * log_timescale_increment).exp();
//         // Calculate the sine and cosine values for this dimension and position
//         row.push((div_term * time).sin());
//         row.push((div_term * time).cos());
//     }

//     row
// }

// fn time_embedding(times: Vec<SeriesFloat>, width: usize) -> Vec<Vec<ModelFloat>> {
//     let d_model = width;
//     let length = times.len();

//     // Calculate the increment for the logarithmic timescale
//     let log_timescale_increment = -(MAX_TIME_SCALE).ln() / d_model as ModelFloat;

//     // Create a vector to hold the sinusoids
//     let mut scaled_time_sin_cos = Vec::with_capacity(length);

//     // Loop over each position in the sequence
//     // for i in 0..length {
//     for time in times {
//         // Create a vector to hold the sinusoids for this position
//         let mut row = Vec::with_capacity(d_model / 2);
//         // Loop over each dimension of the sinusoids
//         for k in (0..d_model).step_by(2) {
//             // Calculate the division term for this dimension
//             let div_term = (k as ModelFloat * log_timescale_increment).exp();
//             // Calculate the sine and cosine values for this dimension and position
//             row.push((div_term * time).sin());
//             row.push((div_term * time).cos());
//         }

//         // Add the sinusoids for this position to the vector
//         scaled_time_sin_cos.push(row);
//     }
//     scaled_time_sin_cos
// }

// // fn time_embed_tensor<B:Backend>(scaled_time_sin_cos: Vec<Vec<f32>>, length: usize, d_model: usize) -> Tensor<B,2> {
// //     // Convert the sinusoids to a tensor and return it
// //     let data = Data::new(
// //         scaled_time_sin_cos.into_iter().flatten().collect(),
// //         [length, d_model].into(),
// //     );

// //     Tensor::<B, 2>::from_data(data.convert(), device)
// // }

fn adjust(x: f32) -> f32 {
    (x - 0.5).clamp(0.0, 1.0)
}
