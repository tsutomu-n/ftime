use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone};
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
    let today_start = Local
        .with_ymd_and_hms(
            now_local.year(),
            now_local.month(),
            now_local.day(),
            0,
            0,
            0,
        )
        .unwrap()
        .with_timezone(&Local);

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
    let date = NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap();
    let start = Local
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap();
    start.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TimeBucket;
    use chrono::Utc;

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
