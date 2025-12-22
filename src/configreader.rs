use serde::Deserialize;
use toml;

//struct to represent the whole config file
#[derive(Deserialize)]
pub struct Cfg {
    colors: Colors,
    barinfo: Starbar,
    modules_left: Option<Vec<String>>,
    modules_right: Option<Vec<String>>,
    modules_center: Option<Vec<String>>,
    cpu: Option<Cpu>,
    clock: Option<Clock>,
    launcher: Option<Menu>,
    wifi: Option<Wifi>,
    //bluetooth: Option<Btooth>,
    //power: Option<Power>,
    // battery: Option<Battery>
}
//function to read the config and parse it into structs

pub fn read_config() -> Cfg {
    let file = std::fs::read_to_string("starbar/config.toml").unwrap();
    let config: Cfg = toml::from_str(&file).unwrap();
    return config;
}
// enum to represent different module shapes
#[derive(Deserialize)]
pub enum Shape {
    Square,
    Slanted,
    Powerline,
    Round,
}

//WIP will parse the info from the config structs to the module structs
fn config_to_modules() {}
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

// struct to represent the traits of the bar
// includes things like shape, postion and bg color
#[derive(Deserialize)]
pub struct Starbar {
    module_shape: Shape,
}

//structs representing configs for individual modules
#[derive(Deserialize)]
pub struct Clock {
    icon: Option<String>,
    format: String,
    tooltip: Option<String>,
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

pub fn test_config() {
    let config = read_config();
    println!("{}", config.colors.bg_color)
}
