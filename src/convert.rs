use ndarray::prelude::*;
use crate::*;

impl Features {
    pub fn try_from(id: EventId, raw: &str) -> Result<Features, FeaturesError> {
        let v:Value = serde_json::from_str(raw)?;
        let bid = try_json_field(&v, "bid")?;
        let bid_size = try_json_field(&v, "bidsz")?;
        let ask = try_json_field(&v, "ask")?;
        let ask_size = try_json_field(&v, "asksz")?;
        return Ok(Features { id, x: array!(bid, bid_size, ask, ask_size) });
    }
}

impl Default for Features {
    fn default() -> Self {
        Features { id: 0, x: Array1::<SeriesFloat>::zeros(NUM_FEATURES) }
    }
}

impl FeaturesSeries {
    pub fn new(id: EventId) -> Self {
        FeaturesSeries { id, x: Array2::<SeriesFloat>::zeros((NUM_FEATURES, SERIES_LENGTH)) }
    }

    pub fn insert(&mut self, idx:usize, features:Features) {
        self.x.slice_mut(s![.., idx]).assign(&features.x);
    }
}

// ----
fn try_json_field(v:&Value, name:&str) -> Result<f32,FeaturesError> {
    let Some(field) = v[name].as_f64() else { return Err(FeaturesError::new(format!("{} not found", name))) };
    return Ok(field as f32);
}

impl FeaturesError {
    fn new(msg: String) -> Self {
        // FeaturesError { under: Box::new(msg) }
        FeaturesError { under: msg }
    }
}

// impl Display for FeaturesError {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         <Box<dyn Display> as Display>::fmt(&self.under, f)
//     }
// }

impl From<serde_json::Error> for FeaturesError {
    fn from(err: serde_json::Error) -> Self {
        // FeaturesError { under: Box::new(err) }
        FeaturesError { under: err.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        let s = FeaturesSeries::new(17);
        assert_eq!(s.id, 17);
    }
}
