//! Live UTC clock: redraws the current time (with seconds) once per second.
//!
//! Runs until interrupted with Ctrl+C. Std-only, so the timestamp is converted
//! from the Unix epoch by hand and always shown in UTC.

use std::io::{self, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const SECS_PER_DAY: u64 = 86_400;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    loop {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        // Carriage return repaints the same line instead of scrolling.
        write!(out, "\r{}", format_utc(now.as_secs()))?;
        out.flush()?;

        // Sleep only up to the next whole second, so the clock never drifts.
        let wait = Duration::from_nanos(1_000_000_000 - u64::from(now.subsec_nanos()));
        std::thread::sleep(wait);
    }
}

/// Formats Unix seconds as `Thu 2026-11-12 14:03:27 UTC`.
fn format_utc(unix_secs: u64) -> String {
    let days = (unix_secs / SECS_PER_DAY) as i64;
    let secs_of_day = unix_secs % SECS_PER_DAY;

    let (year, month, day) = civil_from_days(days);
    let hour = secs_of_day / 3_600;
    let minute = (secs_of_day % 3_600) / 60;
    let second = secs_of_day % 60;

    format!(
        "{} {year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02} UTC",
        weekday_name(days),
    )
}

/// Day of week for days since 1970-01-01 (a Thursday), Sunday-based name.
fn weekday_name(days_since_epoch: i64) -> &'static str {
    let index = (days_since_epoch + 4).rem_euclid(7);
    WEEKDAYS[index as usize]
}

/// Converts days since the Unix epoch to a (year, month, day) civil date.
///
/// Howard Hinnant's `civil_from_days` algorithm (public domain), valid for the
/// proleptic Gregorian calendar.
fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097; // [0, 146096]
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153; // [0, 11], March-based
    let day = (day_of_year - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;

    (if month <= 2 { year + 1 } else { year }, month, day)
}

#[cfg(test)]
mod tests {
    use super::{civil_from_days, format_utc, weekday_name};

    #[test]
    fn epoch_is_a_thursday() {
        assert_eq!(format_utc(0), "Thu 1970-01-01 00:00:00 UTC");
    }

    #[test]
    fn billion_seconds() {
        assert_eq!(format_utc(1_000_000_000), "Sun 2001-09-09 01:46:40 UTC");
    }

    #[test]
    fn leap_day_is_handled() {
        assert_eq!(format_utc(1_709_164_800), "Thu 2024-02-29 00:00:00 UTC");
    }

    #[test]
    fn end_of_day_rolls_over() {
        assert_eq!(format_utc(86_399), "Thu 1970-01-01 23:59:59 UTC");
        assert_eq!(format_utc(86_400), "Fri 1970-01-02 00:00:00 UTC");
    }

    #[test]
    fn civil_round_trip_against_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
    }

    #[test]
    fn weekday_cycles_over_a_week() {
        let names: Vec<_> = (0..7).map(weekday_name).collect();
        assert_eq!(names, ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"]);
    }
}
