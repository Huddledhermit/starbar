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
    colors: modules::Colors,
    barinfo: modules::Starbar,
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


pub fn test_config() {
    let config = read_config();
    println!("{}", config.colors.bg_color)
}
