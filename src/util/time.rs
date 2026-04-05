use chrono::{DateTime, Local, TimeZone, Utc};
use std::time::{Duration, SystemTime};

/// Compute the bucket for a given modification time.
pub fn classify_bucket(now: SystemTime, mtime: SystemTime) -> crate::model::TimeBucket {
    use crate::model::TimeBucket;

    let Ok(elapsed) = now.duration_since(mtime) else {
        return TimeBucket::Active;
    };

    if elapsed < Duration::from_secs(3600) {
        return TimeBucket::Active;
    }

    let now_local: DateTime<Local> = now.into();
    let m_local: DateTime<Local> = mtime.into();
    let today_start = start_of_local_day(now_local);

    if m_local >= today_start {
        return TimeBucket::Today;
    }

    if elapsed < Duration::from_secs(7 * 24 * 3600) {
        return TimeBucket::ThisWeek;
    }

    TimeBucket::History
}

/// Render a compact relative time string for human/plain output.
pub fn relative_time(now: SystemTime, mtime: SystemTime) -> String {
    match now.duration_since(mtime) {
        Ok(elapsed) => compact_elapsed(elapsed, mtime),
        Err(_) => {
            let future = mtime.duration_since(now).unwrap_or(Duration::ZERO);
            compact_future(future)
        }
    }
}

/// Render a local absolute timestamp string for CLI output.
pub fn absolute_time(mtime: SystemTime) -> String {
    let dt: DateTime<Local> = mtime.into();
    format!(
        "{} ({})",
        dt.format("%Y-%m-%d %H:%M:%S"),
        utc_offset_label(dt)
    )
}

pub fn utc_rfc3339(mtime: SystemTime) -> String {
    let dt: DateTime<Utc> = mtime.into();
    dt.to_rfc3339()
}

#[allow(dead_code)]
pub fn start_of_day(ts: SystemTime) -> SystemTime {
    let dt: DateTime<Local> = ts.into();
    start_of_local_day(dt).into()
}

fn start_of_local_day(now: DateTime<Local>) -> DateTime<Local> {
    let date = now.date_naive();
    let Some(naive) = date.and_hms_opt(0, 0, 0) else {
        return now;
    };

    match Local.from_local_datetime(&naive) {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(dt, _) => dt,
        chrono::LocalResult::None => now,
    }
}

fn utc_offset_label(dt: DateTime<Local>) -> String {
    format!("UTC{}", dt.format("%:z"))
}

fn compact_elapsed(elapsed: Duration, mtime: SystemTime) -> String {
    let secs = elapsed.as_secs();
    if secs < 60 {
        return format!("{secs}s");
    }

    let mins = secs / 60;
    if mins < 60 {
        return format!("{mins}m");
    }

    let hours = secs / 3600;
    if hours < 24 {
        return format!("{hours}h");
    }

    let days = secs / 86_400;
    if days < 7 {
        return format!("{days}d");
    }

    let dt: DateTime<Local> = mtime.into();
    dt.format("%Y-%m-%d").to_string()
}

fn compact_future(future: Duration) -> String {
    let secs = future.as_secs();
    if secs < 60 {
        return format!("+{secs}s [skew]");
    }

    let mins = secs / 60;
    if mins < 60 {
        return format!("+{mins}m [skew]");
    }

    let hours = secs / 3600;
    if hours < 24 {
        return format!("+{hours}h [skew]");
    }

    let days = secs / 86_400;
    format!("+{days}d [skew]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TimeBucket;
    use chrono::Duration as ChronoDuration;
    use chrono::{Datelike, Utc};

    fn find_dst_transition_in_year(year: i32) -> Option<(DateTime<Local>, DateTime<Local>)> {
        let start = Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).single()?;
        let end = Local.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).single()?;
        let mut prev = start;
        let mut prev_offset = prev.offset().local_minus_utc();
        let mut cursor = start + ChronoDuration::hours(6);
        while cursor < end {
            let offset = cursor.offset().local_minus_utc();
            if offset != prev_offset {
                return Some((prev, cursor));
            }
            prev = cursor;
            prev_offset = offset;
            cursor += ChronoDuration::hours(6);
        }
        None
    }

    #[test]
    fn test_classify_bucket_boundaries() {
        let now = SystemTime::now();
        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(10)),
            TimeBucket::Active
        );
        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(3599)),
            TimeBucket::Active
        );

        let today_start = start_of_day(now);
        assert_eq!(
            classify_bucket(now, today_start + Duration::from_secs(10)),
            TimeBucket::Today
        );

        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(2 * 86_400)),
            TimeBucket::ThisWeek
        );
        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(8 * 86_400)),
            TimeBucket::History
        );
    }

    #[test]
    fn test_classify_bucket_around_dst_transition() {
        let now_local = Local::now();
        let year = now_local.year();
        let transition =
            find_dst_transition_in_year(year).or_else(|| find_dst_transition_in_year(year - 1));
        let Some((_, after)) = transition else {
            return;
        };

        let now_dt = after + ChronoDuration::hours(6);
        let now: SystemTime = now_dt.into();

        let mtime_dt = Local
            .with_ymd_and_hms(now_dt.year(), now_dt.month(), now_dt.day(), 0, 30, 0)
            .single()
            .filter(|dt| *dt < now_dt)
            .unwrap_or_else(|| now_dt - ChronoDuration::hours(2));

        if mtime_dt.date_naive() == now_dt.date_naive() {
            assert_eq!(classify_bucket(now, mtime_dt.into()), TimeBucket::Today);
        }

        let within = now - Duration::from_secs(7 * 24 * 3600 - 1);
        assert_eq!(classify_bucket(now, within), TimeBucket::ThisWeek);
        let outside = now - Duration::from_secs(7 * 24 * 3600 + 1);
        assert_eq!(classify_bucket(now, outside), TimeBucket::History);
    }

    #[test]
    fn test_relative_time_strings() {
        let now = Utc::now().with_timezone(&Local).into();
        let m = now - Duration::from_secs(12);
        assert_eq!(relative_time(now, m), "12s");
        let m = now - Duration::from_secs(5 * 60);
        assert_eq!(relative_time(now, m), "5m");
        let m = now - Duration::from_secs(3 * 3600);
        assert_eq!(relative_time(now, m), "3h");
        let m = now - Duration::from_secs(6 * 86_400);
        assert_eq!(relative_time(now, m), "6d");
    }

    #[test]
    fn test_relative_time_strings_for_future_skew() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
        assert_eq!(
            relative_time(now, now + Duration::from_secs(45)),
            "+45s [skew]"
        );
        assert_eq!(
            relative_time(now, now + Duration::from_secs(125)),
            "+2m [skew]"
        );
        assert_eq!(
            relative_time(now, now + Duration::from_secs(3 * 3600)),
            "+3h [skew]"
        );
    }

    #[test]
    fn test_absolute_time_format() {
        let ts = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let rendered = absolute_time(ts);
        assert!(rendered.starts_with("20"));
        assert!(rendered.contains(" (UTC"));
        assert!(rendered.ends_with(')'));
    }
}
