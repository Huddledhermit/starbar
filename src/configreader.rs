use std::string;

use serde::Deserialize;
use toml::Table;
struct cfg {
    colors: string,
}
fn read_config(file: String) {
    let file = std::fs::read_to_string("starbar/config.toml");
    let config: Table = file.parse().unwrap();
}
