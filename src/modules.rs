use crate::configreader;
use chrono::{DateTime, Local};
use serde::Deserialize;
use std::string;
use toml;

#[derive(serde::Deserialize)]
pub struct Clock {
    pub icon: Option<String>,
}

impl Clock {
    pub fn get_time(&self, time_format: &str) -> String {
        let time_no_format = chrono::Local::now();
        format!("{}", time_no_format.format(time_format))
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
    fn on_press(com: &str) {
        let mut parsed_command: Vec<&str> = com.split(" ").collect();
        let mut command = std::process::Command::new(parsed_command[0]);
        parsed_command.remove(0);
        for i in parsed_command {
            command.arg(i);
        }
        command.output();
    }
}
