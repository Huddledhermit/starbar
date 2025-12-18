use serde::Deserialize;
use toml::Table;

//struct to represent the whole config file
#[derive(Deserialize)]
struct Cfg {
    colors: String,
    barinfo: String,
    modules_left: Option<String>,
    modules_right: Option<String>,
    modules_center: Option<String>,
    cpu: Option<String>,
    clock: Option<clock>,
    launcher: Option<String>,
    wifi: Option<String>,
}
//function to read the config and parse it into structs
fn read_config(file: String) {
    let file = std::fs::read_to_string("starbar/config.toml");
    let config: Table = file.parse().unwrap();
}
//WIP will parse the info from the config structs to the module structs
fn config_to_modules() {}

//structs representing configs for individual modules
#[derive(Deserialize)]
struct clock {
    icon: Option<String>,
    format: String,
    tooltip: Option<String>,
}
