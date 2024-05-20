use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

use crate::*;

const INVALID_DATE: NaiveDate = NaiveDate::MIN;

pub fn same_date(date1: NaiveDate, date2: NaiveDate) -> bool {
    date1 != INVALID_DATE && date2 != INVALID_DATE && date1 == date2
}

pub fn event_in_trading_time(ev: &QuoteEvent) -> bool { // anyhow::Result<bool> {
    // let ev: QuoteEvent = msg_to(msg)?;
    ts_in_trading_time(ev.biddate) && ts_in_trading_time(ev.askdate)
}

pub fn valid_time_and_date(ev: &QuoteEvent, date: NaiveDate) -> bool {
    same_date(to_date(ev), date) && event_in_trading_time(ev)
}

pub fn now() -> Timestamp {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid system time").as_millis() as Timestamp
}

pub fn to_datetime(millis: Timestamp) -> UtcDateTime {
    // We don't need to convert to UTC first as the warning says because we always have it UTC for NaiveDateTime.
    #[allow(deprecated)]
    NaiveDateTime::from_timestamp_millis(millis).unwrap()
}

pub fn to_date(ev: &QuoteEvent) -> NaiveDate {
    let bid_date = to_market_datetime(ev.biddate).date_naive();
    let ask_date = to_market_datetime(ev.askdate).date_naive();
    // TODO: if they're very near each other, could choose one, probably latter
    // arbitrary 10 seconds?
    if bid_date == ask_date || (ev.askdate - ev.biddate) < 10 {
        ask_date
    } else {
        // bail!("bid date and ask date are not the same");
        INVALID_DATE
    }
}

pub fn to_market_datetime(millis: Timestamp) -> DateTime<chrono_tz::Tz> {
    DateTime::from_timestamp_millis(millis).unwrap().with_timezone(&MARKET_TIMEZONE)
}

/// Currently just checks if it is a weekday within typical eastern tz trading hours.
/// TODO: could use calendar data acquired from tradier
pub fn dt_in_trading_time(dt: DateTime<chrono_tz::Tz>) -> bool {
    let istime = (NaiveTime::from_hms_opt(9,30,0)..NaiveTime::from_hms_opt(16,0,0)).contains(&Some(dt.time()));
    is_weekday(dt) && istime
}

pub fn ts_in_trading_time(ts: Timestamp) -> bool {
    dt_in_trading_time(to_market_datetime(ts))
    // let dt = to_market_datetime(ts);
}

fn is_weekday<Tz: TimeZone>(dt: DateTime<Tz>) -> bool {
    dt.weekday().num_days_from_monday() < 5
}
