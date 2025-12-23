use std::string;
use serde::Deserialize;
use toml;
use chrono::{DateTime, Local};
use iced;
use crate::configreader;

#[derive(Deserialize)]
pub enum Shape {
    Square,
    Slanted,
    Powerline,
    Round,
}

#[derive(Deserialize)]
pub struct Starbar {
    module_shape: Shape,
}


#[derive(Deserialize)]
pub struct Colors {
    bg_color: String,
    fg_color: String,
    color3: Option<String>,
    color4: Option<String>,
    color5: Option<String>,
    color6: Option<String>,
    color7: Option<String>,
    color8: Option<String>,
}


#[derive(Deserialize)]
pub struct Clock {
    icon: Option<String>,
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


#[derive(Deserialize)]
pub struct Cpu {
    icon: String,
    usage: i32,
    format: String,
    tooltip: Option<String>,
}


#[derive(Deserialize)]
pub struct Wifi {
    icon_connected: String,
    icon_discon: String,
}

#[derive(Deserialize)]
pub struct Battery {
    icons: Vec<String>,
    format: String,
}
#[derive(Deserialize)]
pub struct Menu {
    icon: String,
    exec: String,
}
