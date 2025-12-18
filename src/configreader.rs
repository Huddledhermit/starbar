use serde::Deserialize;
use toml::Table;

#[derive(Deserialize)]
struct cfg {
    colors: string,
    barinfo: string,
    modules-left:  string,
    modules-right: string,
    modules-center: string,
    cpu: string,
    clock: string,
    launcher: string,
    wifi: string

}
fn read_config(file: String) {
    let file = std::fs::read_to_string("starbar/config.toml");
    let config: Table = file.parse().unwrap();
}
