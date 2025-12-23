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
   pub  module_shape: Shape,
}


#[derive(Deserialize)]
pub struct Colors {
    pub bg_color: String,
    pub fg_color: String,
    pub color3: Option<String>,
    pub color4: Option<String>,
    pub color5: Option<String>,
    pub color6: Option<String>,
    pub color7: Option<String>,
    pub color8: Option<String>,
}


#[derive(Deserialize)]
pub struct Clock {
   pub icon: Option<String>,
    pub time: String,
    pub tooltip: bool,
    pub tipvalue: Option<String>,
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
    pub icon: String,
    pub usage: i32,
    pub format: String,
    pub tooltip: Option<String>,
}


#[derive(Deserialize)]
pub struct Wifi {
    pub icon_connected: String,
    pub icon_discon: String,
}

#[derive(Deserialize)]
pub struct Battery {
    pub icons: Vec<String>,
    pub format: String,
}
#[derive(Deserialize)]
pub struct Menu {
    pub icon: String,
    pub exec: String,
}

impl Menu {
    fn on_press(){

    }

}
