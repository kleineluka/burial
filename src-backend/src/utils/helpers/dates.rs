// get the current date in string format (dd-mm-yyyy)
#[allow(dead_code)]
pub fn get_date() -> String {
    let now = chrono::Utc::now();
    let date = now.format("%d-%m-%Y").to_string();
    date
}

// parse string into date (dd-mm-yyyy)
#[allow(dead_code)]
pub fn parse_date(date: String) -> chrono::DateTime<chrono::Utc> {
    let date = chrono::NaiveDate::parse_from_str(&date, "%d-%m-%Y").unwrap();
    let date = chrono::DateTime::<chrono::Utc>::from_utc(date.and_hms(0, 0, 0), chrono::Utc);
    date
}

// take in a date (dd-mm-yyyy), get current date, and see how many days have passed
#[allow(dead_code)]
pub fn days_passed(date: String) -> i64 {
    let date = parse_date(date);
    let now = chrono::Utc::now();
    let days = now.signed_duration_since(date).num_days();
    days
}