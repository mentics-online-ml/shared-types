use crate::*;
use chrono_util::*;
use series::*;
use series_proc::BaseValues;

/// Published to series by ingest and read by label, train...
#[derive(Debug, serde::Deserialize)]
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

impl QuoteEvent {
    fn event_in_trading_time(&self) -> bool {
        ts_in_trading_time(self.biddate) && ts_in_trading_time(self.askdate)
    }

    fn to_date_or_0(&self) -> NaiveDate {
        let bid_date = to_market_datetime(self.biddate).date_naive();
        let ask_date = to_market_datetime(self.askdate).date_naive();
        // TODO: if they're very near each other, could choose one, probably latter
        // arbitrary 10 seconds?
        if bid_date == ask_date || (self.askdate - self.biddate) < 10 {
            ask_date
        } else {
            // bail!("bid date and ask date are not the same");
            INVALID_DATE
        }
    }
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

impl SeriesEvent for QuoteEvent {
    type BV = QuoteValues;

    fn set_ids(&mut self, event_id: EventId, offset: OffsetId) {
        self.event_id = event_id;
        self.offset = offset;
    }

    fn timestamp(&self) -> Timestamp {
        // TODO: validate the timestamps are similar
        self.biddate
    }

    fn validity(&self, base: &Self::BV) -> Validity {
        if !self.event_in_trading_time() {
            Validity::Invalid
        } else if !same_date(self.to_date_or_0(), base.date_or_0) {
            Validity::CauseReset
        } else {
            Validity::Valid
        }
    }
}

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
