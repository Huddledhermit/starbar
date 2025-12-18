use serde::Deserialize;
use toml::Table;

//struct to represent the whole config file
#[derive(Deserialize)]
struct Cfg {
    colors: String,
    //barinfo: starbar,
    modules_left: Option<vec<String>>,
    modules_right: Option<vec<String>>,
    modules_center: Option<vec<String>>,
    cpu: Option<cpu>,
    clock: Option<clock>,
  //  launcher: Option<menu>,
    wifi: Option<wifi>,
}
//function to read the config and parse it into structs
fn read_config(file: String) {
    let file = std::fs::read_to_string("starbar/config.toml");
    let config: Cfg = Toml::from_str(file).unwrap();
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

#[derive(Deserialize)]
struct cpu{
    icon: String,
    usage: i32,
    format: String,
    tooltip: Option<String>
}
#[derive(Deserialize)]
struct wifi{

}
