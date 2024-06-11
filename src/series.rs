use crate::*;

pub trait EventType = SeriesEvent + DeserializeOwned;

pub enum Validity {
    Valid,
    CauseReset,
    Invalid,
}

pub trait SeriesEvent {
    type BV;

    fn set_ids(&mut self, event_id: EventId, offset: OffsetId);
    fn timestamp(&self) -> Timestamp;
    fn validity(&self, base: &Self::BV) -> Validity;

    // fn event_in_trading_time(&self) -> bool {
    //     ts_in_trading_time(self.timestamp())
    // }

    // fn to_date_or_0(&self) -> NaiveDate {
    //     to_market_datetime(self.timestamp()).date_naive()
    // }
}
