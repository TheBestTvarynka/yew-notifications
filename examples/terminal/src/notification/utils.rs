use time::Time;

pub fn format_time(time: &Time) -> String {
    format!("{:02?}:{:02?}:{:02?}", time.hour(), time.minute(), time.second())
}
