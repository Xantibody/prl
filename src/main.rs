mod config;
use chrono::{Datelike, NaiveDate};
fn main() {
    let name = "aizawa";
    let birth = NaiveDate::from_ymd_opt(1990, 01, 01).expect("invalid birth");
    let avg_years: i32 = 80;
    let projected = birth.with_year(birth.year() + avg_years).unwrap();
    let today = NaiveDate::from_ymd_opt(2035, 1, 1).unwrap();
    let mut age = today.year() - birth.year();
    if (today.month(), today.day()) < (birth.month(), birth.day()) {
        age -= 1;
    }
    let remaining = (avg_years - age).max(0);
    println!("{} {} {}", name, projected, remaining);
}
