use std::string;
use serde::Deserialize;
use toml;
use chrono::{DateTime, Local};
use iced;
use crate::configreader;

#[derive(Deserialize)]
pub struct Clock {
    text: String,
    time: String,
    tooltip: bool,
    tipvalue: Option<String>,
}


impl Clock {
    pub fn get_time(&self) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format("%H:%M|%d/%m"))
    }
    pub fn Update_Time(&mut self){
        let new_time = self.get_time();
        self.time = new_time;
    }
}
