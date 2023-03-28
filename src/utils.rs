#[cfg(feature = "standard-notification")]
pub fn format_date_time(datetime: &time::OffsetDateTime) -> String {
    format!(
        "{}:{}:{} {}.{}.{}",
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.day(),
        datetime.month(),
        datetime.year()
    )
}
