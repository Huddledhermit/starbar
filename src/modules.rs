use std::string;

use chrono::{DateTime, Local};
use iced;

pub struct Clock {
    text: String,
    tooltip: bool,
    tipvalue: Option<String>,
}
impl Clock {
    fn new(mut self, display: String, tooltip: bool) -> Clock {
        self.text = display;
        if self.tooltip == false {
            self.tipvalue = None
        } else {
        };
        self
    }
    fn get_time(self) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format("%H:%M|%d/%m"))
    }
}
pub struct Cpu {}

pub struct Panel {}
