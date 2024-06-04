use chrono::NaiveDate;

use crate::{*,util::*};

// impl QuoteEvent {
// }

impl SeriesEvent for QuoteEvent {
    fn set_ids(&mut self, event_id: EventId, offset: OffsetId) {
        self.event_id = event_id;
        self.offset = offset;
    }

    fn timestamp(&self) -> Timestamp {
        // TODO: validate the timestamps are similar
        self.biddate
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

    fn event_in_trading_time(&self) -> bool {
        ts_in_trading_time(self.biddate) && ts_in_trading_time(self.askdate)
    }
}
