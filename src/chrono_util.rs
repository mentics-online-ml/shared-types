use std::time::{SystemTime, UNIX_EPOCH};
// use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono::prelude::*;
use num_traits::AsPrimitive;

use crate::*;

pub const MARKET_TIMEZONE: chrono_tz::Tz = chrono_tz::US::Eastern;
pub const INVALID_DATE: NaiveDate = NaiveDate::MIN;

pub fn same_date(date1: NaiveDate, date2: NaiveDate) -> bool {
    date1 != INVALID_DATE && date2 != INVALID_DATE && date1 == date2
}

// pub fn valid_time_and_date(ev: &QuoteEvent, date: NaiveDate) -> bool {
//     same_date(to_date(ev), date) && event_in_trading_time(ev)
// }

pub fn now() -> Timestamp {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Invalid system time").as_millis() as Timestamp
}

pub fn to_datetime(millis: Timestamp) -> UtcDateTime {
    // We don't need to convert to UTC first as the warning says because we always have it UTC for NaiveDateTime.
    #[allow(deprecated)]
    NaiveDateTime::from_timestamp_millis(millis).unwrap()
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

pub const CHRONO_FEATURES_SIZE: usize = 7;
pub type ChronoFeatures = [ModelFloat; CHRONO_FEATURES_SIZE];
pub const CHRONO_BYTE_SIZE: usize = std::mem::size_of::<ChronoFeatures>();

// TODO: consider embedding time as a function of year with many sinusiods instead
pub fn make_chrono_features(timestamp: Timestamp) -> ChronoFeatures {
    let date_time = DateTime::<Utc>::from_timestamp_millis(timestamp).unwrap();
    let naive_date = date_time.naive_utc().date();
    let second = embed(date_time.second() + 1, 60.0);
    let minute = embed(date_time.minute() + 1, 60.0);
    let hour = embed(date_time.hour() + 1, 24.0);
    let day_of_week = embed(date_time.weekday() as u8 + 1, 7.0);
    let day_of_month = embed(date_time.day(), num_days_in_month(date_time.year(), date_time.month()));
    let day_of_quarter = embed(day_of_quarter(naive_date), num_days_in_quarter(naive_date));
    // let day_of_year = embed(day_of_year(naive_date), num_days_in_year(naive_date));
    let month = embed(date_time.month(), 12.0);
    // embed(day_of_year(naive_date), num_days_in_year(naive_date));
    [second, minute, hour, day_of_week, day_of_month, day_of_quarter, month]
}

fn embed<N: AsPrimitive<f32>, M: AsPrimitive<f32>>(x: N, max_val: M) -> ModelFloat {
    x.as_() / max_val.as_()
}

// from https://stackoverflow.com/a/58188385/315734
// unwraps are safe because logic
pub fn num_days_in_month(year: i32, month: u32) -> i64 {
    let first_of_next_month = NaiveDate::from_ymd_opt(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    ).unwrap();
    first_of_next_month.signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap()).num_days()
}

// unwraps are safe because logic
// pub fn day_of_year(date: NaiveDate) -> i64 {
//     let start_of_year = NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();
//     date.signed_duration_since(start_of_year).num_days()
// }

// pub fn num_days_in_year(date: NaiveDate) -> i64 {
//     let start_of_year = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
//     let start_of_next_year = NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap();
//     start_of_next_year.signed_duration_since(start_of_year).num_days()
// }

pub fn day_of_quarter(date: NaiveDate) -> i64 {
    let res = date.signed_duration_since(first_day_of_quarter(date)).num_days();
    // println!("day_of_quarter: {} -> {}", date, res);
    res
}

pub fn num_days_in_quarter(date: NaiveDate) -> i64 {
    let first_of_quarter = first_day_of_quarter(date);
    let from_month = first_of_quarter.month();
    let (add_to_year, next_quarter_month_minus_one) = util::divrem(from_month - 1 + 3, 12);
    let to_year = first_of_quarter.year() + add_to_year as i32; // will be 0 unless beyond 12
    let first_of_next_quarter = NaiveDate::from_ymd_opt(to_year, next_quarter_month_minus_one + 1, 1).unwrap();
    let res = first_of_next_quarter.signed_duration_since(first_of_quarter).num_days();
    // println!("num_days_in_quarter: {} -> {}", date, res);
    res
}

pub fn first_day_of_quarter(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), first_month_in_quarter(date.month()), 1).unwrap()
}

fn first_month_in_quarter(month: u32) -> u32 {
    (month - 1) % 3 + 1
}

// fn month_of_quarter(date: &NaiveDate) -> u32 {
//     1 + 3 * ((date.month() - 1) / 3)
// }
