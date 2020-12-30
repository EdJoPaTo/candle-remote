use chrono::{DateTime, TimeZone};
use std::time::Duration;

#[allow(clippy::cast_precision_loss)]
pub fn calc_relative_position(start: i64, end: i64, position: i64) -> f64 {
    let relative_max = end - start;
    let relative_position = position - start;
    relative_position as f64 / relative_max as f64
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn interpolate(start: i64, end: i64, position: f64) -> i64 {
    let relative_max = end - start;
    let relative_position = relative_max as f64 * position;
    start + relative_position as i64
}

pub fn duration_until<TzFrom, TzUntil>(
    from: &DateTime<TzFrom>,
    until: &DateTime<TzUntil>,
) -> Option<Duration>
where
    TzFrom: TimeZone,
    TzUntil: TimeZone,
{
    let from = from.timestamp_nanos();
    let until = until.timestamp_nanos();

    let duration_in_nanos = until - from;
    if duration_in_nanos > 0 {
        #[allow(clippy::cast_sign_loss)]
        Some(Duration::from_nanos(duration_in_nanos as u64))
    } else {
        None
    }
}

#[test]
fn calc_relative_position_start() {
    assert!((0.0 - calc_relative_position(2, 4, 2)).abs() < f64::EPSILON);
}
#[test]
fn calc_relative_position_end() {
    assert!((1.0 - calc_relative_position(2, 4, 4)).abs() < f64::EPSILON);
}
#[test]
fn calc_relative_position_half() {
    assert!((0.5 - calc_relative_position(2, 4, 3)).abs() < f64::EPSILON);
}

#[test]
fn interpolate_start() {
    assert_eq!(2, interpolate(2, 4, 0.0));
}

#[test]
fn interpolate_end() {
    assert_eq!(4, interpolate(2, 4, 1.0));
}

#[test]
fn interpolate_half() {
    assert_eq!(3, interpolate(2, 4, 0.5));
}

#[test]
fn duration_until_past_is_none() {
    let from = DateTime::parse_from_rfc3339("2020-10-12T20:02:00+00:00").unwrap();
    let until = DateTime::parse_from_rfc3339("2020-10-12T20:00:00+00:00").unwrap();
    assert_eq!(None, duration_until(&from, &until));
}

#[test]
fn duration_until_future_works() {
    let from = DateTime::parse_from_rfc3339("2020-10-12T20:00:00+00:00").unwrap();
    let until = DateTime::parse_from_rfc3339("2020-10-12T20:02:00+00:00").unwrap();
    assert_eq!(
        Some(Duration::from_secs(120)),
        duration_until(&from, &until)
    );
}
