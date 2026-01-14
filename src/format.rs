/// Format remaining time using format specifiers
///
/// Specifiers:
/// - %D: Total days
/// - %H: Total hours
/// - %M: Total minutes
/// - %Y: Years
/// - %d: Days (remainder after years)
/// - %h: Hours (remainder after days)
/// - %m: Minutes (remainder after hours)
pub fn format_remaining(total_days: u64, fmt: &str) -> String {
    let total_hours = total_days * 24;
    let total_minutes = total_hours * 60;

    let years = total_days / 365;
    let days_after_years = total_days % 365;
    let hours_after_days = 0_u64; // Days are whole, no remainder
    let minutes_after_hours = 0_u64;

    fmt.replace("%D", &total_days.to_string())
        .replace("%H", &total_hours.to_string())
        .replace("%M", &total_minutes.to_string())
        .replace("%Y", &years.to_string())
        .replace("%d", &days_after_years.to_string())
        .replace("%h", &hours_after_days.to_string())
        .replace("%m", &minutes_after_hours.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_total_days() {
        assert_eq!(format_remaining(42, "%D"), "42");
    }

    #[test]
    fn format_total_hours() {
        assert_eq!(format_remaining(2, "%H"), "48");
    }

    #[test]
    fn format_total_minutes() {
        assert_eq!(format_remaining(1, "%M"), "1440");
    }

    #[test]
    fn format_years_and_days() {
        // 400 days = 1 year + 35 days
        assert_eq!(format_remaining(400, "%Y年%d日"), "1年35日");
    }

    #[test]
    fn format_combined() {
        assert_eq!(format_remaining(42, "%D days remaining"), "42 days remaining");
    }

    #[test]
    fn format_zero_days() {
        assert_eq!(format_remaining(0, "%D"), "0");
    }
}
