/// Formats a date string into a more readable format.
///
/// # Panics
///
/// Panics if the input date string does not match the expected datetime format.
pub fn format_post_date(date_str: &str) -> String {
    use chrono::NaiveDateTime;
    use domain::constants::{DATETIME_FORMAT, POST_DATE_FORMAT};

    NaiveDateTime::parse_from_str(date_str, DATETIME_FORMAT)
        .expect("Failed to parse date")
        .format(POST_DATE_FORMAT)
        .to_string()
}
