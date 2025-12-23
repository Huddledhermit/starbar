use serde::Deserialize;
use toml;
use crate::modules;


pub fn read_config() -> Cfg {
    let file = std::fs::read_to_string("starbar/config.toml").unwrap();
    let config: Cfg = toml::from_str(&file).unwrap();
    return config;
}


#[derive(Deserialize)]
pub struct Cfg {
    colors: Colors,
    barinfo: Starbar,
    modules_left: Option<Vec<String>>,
    modules_right: Option<Vec<String>>,
    modules_center: Option<Vec<String>>,
    cpu: Option<modules::Cpu>,
    clock: Option<modules::Clock>,
    launcher: Option<modules::Menu>,
    wifi: Option<modules::Wifi>,
    //bluetooth: Option<Btooth>,
    //power: Option<Power>,
    // battery: Option<Battery>
}


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

pub fn test_config() {
    let config = read_config();
    println!("{}", config.colors.bg_color)
}
