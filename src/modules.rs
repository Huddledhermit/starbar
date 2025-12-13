use std::string;

use chrono::{DateTime, Local};
use iced;

pub struct Clock {
    text: String,
    time: String,
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
    fn get_time(&self) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format("%H:%M|%d/%m"))
    }
    fn Update_Time(&mut self){
        let new_time = self.get_time();
        self.time = new_time;
    }
}
pub struct Cpu {}

pub struct Panel {}
