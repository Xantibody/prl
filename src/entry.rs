use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct DateEntry {
    pub date: NaiveDate,
}

impl DateEntry {
    pub fn new(date: NaiveDate) -> Self {
        Self { date }
    }

    pub fn remaining_days(&self, today: NaiveDate) -> u64 {
        let diff = self.date.signed_duration_since(today).num_days();
        if diff < 0 { 0 } else { diff as u64 }
    }

    pub fn remaining_hours(&self, today: NaiveDate) -> u64 {
        self.remaining_days(today) * 24
    }

    pub fn remaining_minutes(&self, today: NaiveDate) -> u64 {
        self.remaining_hours(today) * 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn remaining_days_future() {
        let entry = DateEntry::new(date(2025, 1, 20));
        let today = date(2025, 1, 15);
        assert_eq!(entry.remaining_days(today), 5);
    }

    #[test]
    fn remaining_days_past_returns_zero() {
        let entry = DateEntry::new(date(2025, 1, 10));
        let today = date(2025, 1, 15);
        assert_eq!(entry.remaining_days(today), 0);
    }

    #[test]
    fn remaining_days_same_day() {
        let entry = DateEntry::new(date(2025, 1, 15));
        let today = date(2025, 1, 15);
        assert_eq!(entry.remaining_days(today), 0);
    }

    #[test]
    fn remaining_hours() {
        let entry = DateEntry::new(date(2025, 1, 17));
        let today = date(2025, 1, 15);
        assert_eq!(entry.remaining_hours(today), 48);
    }

    #[test]
    fn remaining_minutes() {
        let entry = DateEntry::new(date(2025, 1, 16));
        let today = date(2025, 1, 15);
        assert_eq!(entry.remaining_minutes(today), 1440);
    }
}
