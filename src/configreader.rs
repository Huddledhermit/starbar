use serde::Deserialize;
use toml::Table;

//struct to represent the whole config file
#[derive(Deserialize)]
struct cfg {
    colors: String,
    barinfo: String,
    modules-left:  string,
    modules-right: string,
    modules-center: string,
    cpu: string,
    clock: string,
    launcher: string,
    wifi: string
}
//function to read the config and parse it into structs
fn read_config(file: String) {
    let file = std::fs::read_to_string("starbar/config.toml");
    let config: Table = file.parse().unwrap();
}
//WIP will parse the info from the config structs to the module structs
fn config_to_modules(){}

//structs representing configs for individual modules
struct clock{
    icon: Option<String>,
    format: String,
    tooltip: Option<String>,
}
