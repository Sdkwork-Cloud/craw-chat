use std::time::{SystemTime, UNIX_EPOCH};

use crate::TimelineProjectionService;

pub(crate) fn projection_update_delay_ms(source_timestamp: &str) -> Option<u64> {
    let source_epoch_millis = parse_rfc3339_timestamp_millis(source_timestamp)?;
    let now_epoch_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_millis();
    Some(
        now_epoch_millis
            .saturating_sub(source_epoch_millis)
            .min(u64::MAX as u128) as u64,
    )
}

fn parse_rfc3339_timestamp_millis(timestamp: &str) -> Option<u128> {
    let trimmed = timestamp.strip_suffix('Z')?;
    let (date, time) = trimmed.split_once('T')?;
    let (year, month, day) = parse_rfc3339_date(date)?;
    let (hour, minute, second, millis) = parse_rfc3339_time(time)?;
    let days = days_from_civil(year, month, day)?;
    let day_millis = ((hour as i64 * 60 * 60) + (minute as i64 * 60) + second as i64)
        .checked_mul(1000)?
        .checked_add(millis as i64)?;
    let epoch_millis = days.checked_mul(86_400_000)?.checked_add(day_millis)?;
    u128::try_from(epoch_millis).ok()
}

fn parse_rfc3339_date(date: &str) -> Option<(i32, u32, u32)> {
    let mut parts = date.split('-');
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<u32>().ok()?;
    let day = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) {
        return None;
    }
    let max_day = days_in_month(year, month);
    if day == 0 || day > max_day {
        return None;
    }
    Some((year, month, day))
}

fn parse_rfc3339_time(time: &str) -> Option<(u32, u32, u32, u32)> {
    let (hhmmss, fraction) = if let Some((base, fraction)) = time.split_once('.') {
        (base, Some(fraction))
    } else {
        (time, None)
    };
    let mut parts = hhmmss.split(':');
    let hour = parts.next()?.parse::<u32>().ok()?;
    let minute = parts.next()?.parse::<u32>().ok()?;
    let second = parts.next()?.parse::<u32>().ok()?;
    if parts.next().is_some() || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let millis = parse_fraction_millis(fraction)?;
    Some((hour, minute, second, millis))
}

fn parse_fraction_millis(fraction: Option<&str>) -> Option<u32> {
    let Some(fraction) = fraction else {
        return Some(0);
    };
    if fraction.is_empty() || !fraction.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let mut millis = fraction.chars().take(3).collect::<String>();
    while millis.len() < 3 {
        millis.push('0');
    }
    millis.parse::<u32>().ok()
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return None;
    }
    let year = i64::from(year) - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_param = i64::from(month) + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * month_param + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Some(era * 146_097 + day_of_era - 719_468)
}

impl TimelineProjectionService {
    pub(crate) fn record_projection_update_delay_for_scope(
        &self,
        source_event_type: &str,
        scope_id: &str,
        source_timestamp: &str,
    ) {
        if let Some(delay_ms) = projection_update_delay_ms(source_timestamp) {
            self.record_projection_update_delay(source_event_type, scope_id, delay_ms, delay_ms);
        }
    }
}
