use anyhow::{bail, Context};
use ndarray::prelude::*;
use crate::*;

impl Event {
    pub fn try_from(id: EventId, raw: &str) -> anyhow::Result<Event> {
        let v:Value = serde_json::from_str(raw)?;
        let bid = try_json_field(&v, "bid")?;
        let bid_size = try_json_field(&v, "bidsz")?;
        let ask = try_json_field(&v, "ask")?;
        let ask_size = try_json_field(&v, "asksz")?;
        Ok(Event { id, x: array!(bid, bid_size, ask, ask_size) })
    }
}

impl Default for Event {
    fn default() -> Self {
        Event { id: 0, x: Array1::<SeriesFloat>::zeros(NUM_FEATURES) }
    }
}

impl EventSeries {
    pub fn new(id: EventId) -> Self {
        EventSeries { id, x: Array2::<SeriesFloat>::zeros((NUM_FEATURES, SERIES_LENGTH)) }
    }

    pub fn insert(&mut self, idx:usize, features:Event) {
        self.x.slice_mut(s![.., idx]).assign(&features.x);
    }
}

#[allow(deprecated)]
pub fn serialize_timestamp(timestamp: UtcDateTime) -> Timestamp {
    // We don't need to convert to UTC first as the warning says because we always have it UTC for NaiveDateTime.
    timestamp.timestamp_millis()
}

// ----

fn try_json_field(v:&Value, name:&str) -> anyhow::Result<SeriesFloat> {
    let field = &v[name];
    if field.is_null() {
        bail!("Field {} not found in value {:?}", name, v)
    } else {
        field.as_f64().map(|x| x as SeriesFloat).with_context(|| format!("Could not convert {field} to f64"))
    }
}
