use crate::model::Label;
use chrono::{DateTime, Local, TimeZone};
use std::time::{Duration, SystemTime};

/// Freshラベル判定のウィンドウ（秒）
pub const FRESH_WINDOW_SECS: u64 = 5 * 60;

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

/// Render a human-readable relative time string.
pub fn relative_time(now: SystemTime, mtime: SystemTime) -> String {
    let Ok(elapsed) = now.duration_since(mtime) else {
        return "just now".to_string();
    };
    let mins = elapsed.as_secs() / 60;
    let hours = elapsed.as_secs() / 3600;
    let days = elapsed.as_secs() / 86_400;

    if elapsed.as_secs() < 60 {
        "just now".to_string()
    } else if mins < 60 {
        if mins == 1 {
            "1 min ago".to_string()
        } else {
            format!("{} mins ago", mins)
        }
    } else if hours < 24 {
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if days == 1 {
        "Yesterday".to_string()
    } else if days < 7 {
        format!("{} days ago", days)
    } else {
        let dt: DateTime<Local> = mtime.into();
        dt.format("%Y-%m-%d").to_string()
    }
}

/// Truncate time to date for comparisons.
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

/// Best-effort label classification. Currently only `Fresh` with a small time window.
pub fn classify_label(now: SystemTime, mtime: SystemTime) -> Option<Label> {
    let window = Duration::from_secs(FRESH_WINDOW_SECS);
    if now.duration_since(mtime).unwrap_or(Duration::MAX) <= window {
        Some(Label::Fresh)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TimeBucket;
    use chrono::Duration as ChronoDuration;
    use chrono::{Datelike, Utc};

    fn find_dst_transition_in_year(year: i32) -> Option<(DateTime<Local>, DateTime<Local>)> {
        let start = Local.with_ymd_and_hms(year, 1, 1, 0, 0, 0).single()?;
        let end = Local
            .with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0)
            .single()?;
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
            cursor = cursor + ChronoDuration::hours(6);
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

        // Today boundary
        let today_start = start_of_day(now);
        assert_eq!(
            classify_bucket(now, today_start + Duration::from_secs(10)),
            TimeBucket::Today
        );

        // This week boundary
        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(2 * 86_400)),
            TimeBucket::ThisWeek
        );
        assert_eq!(
            classify_bucket(now, now - Duration::from_secs(6 * 86_400)),
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
        let m = now - Duration::from_secs(30);
        assert_eq!(relative_time(now, m), "just now");
        let m = now - Duration::from_secs(5 * 60);
        assert_eq!(relative_time(now, m), "5 mins ago");
        let m = now - Duration::from_secs(3600);
        assert_eq!(relative_time(now, m), "1 hour ago");
        let m = now - Duration::from_secs(3 * 3600);
        assert_eq!(relative_time(now, m), "3 hours ago");
        let m = now - Duration::from_secs(86_400);
        assert_eq!(relative_time(now, m), "Yesterday");
        let m = now - Duration::from_secs(3 * 86_400);
        assert_eq!(relative_time(now, m), "3 days ago");
    }
}
