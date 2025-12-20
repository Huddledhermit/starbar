use serde::Deserialize;
use toml;

//struct to represent the whole config file
#[derive(Deserialize)]
struct Cfg {
    colors: String,
    //barinfo: starbar,
    modules_left: Option<Vec<String>>,
    modules_right: Option<Vec<String>>,
    modules_center: Option<Vec<String>>,
    cpu: Option<Cpu>,
    clock: Option<Clock>,
    //  launcher: Option<Menu>,
    wifi: Option<Wifi>,
    //bluetooth: Option<Btooth>,
    //power: Option<Power>,
    // battery: Option<Battery>
}
//function to read the config and parse it into structs
fn read_config() -> Cfg {
    let file = std::fs::read_to_string("starbar/config.toml").unwrap();
    toml::from_str(&file).unwrap()
}
// enum to represent different module shapes
#[derive(Deserialize)]
enum Shape {
    Square,
    Slanted,
    Powerline,
    Round,
}

//WIP will parse the info from the config structs to the module structs
fn config_to_modules() {}

// struct to represent the traits of the bar
// includes things like shape, postion and bg color
#[derive(Deserialize)]
struct Starbar {
    module_shape: Shape,
}

//structs representing configs for individual modules
#[derive(Deserialize)]
struct Clock {
    icon: Option<String>,
    format: String,
    tooltip: Option<String>,
}

#[derive(Deserialize)]
struct Cpu {
    icon: String,
    usage: i32,
    format: String,
    tooltip: Option<String>,
}
#[derive(Deserialize)]
struct Wifi {
    icon_connected: String,
    icon_discon: String,
}
#[derive(Deserialize)]
struct Battery {
    icons: Vec<String>,
    format: String,
}
#[derive(Deserialize)]
struct Menu {
    icon: String,
    exec: Option<String>,
}
