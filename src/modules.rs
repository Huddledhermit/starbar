use std::string;

use chrono::{DateTime, Local};
use iced;

pub struct Clock {
    text: String,
    tooltip: String,
}
impl Clock {
    fn get_time(self) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format("%H:%M|%d/%m"))
    }
}
pub struct Cpu {}

pub struct Panel {}
