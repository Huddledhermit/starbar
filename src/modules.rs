use chrono::{DateTime, Local};
use iced;

struct Clock {
    date_time: DateTime<Local>,
}
impl clock {
    fn get_time(self) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format("%H:%M|%d/%m"))
    }
}
struct Cpu {}

struct Panel {}
