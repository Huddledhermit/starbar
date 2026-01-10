use gio::prelude::*;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
mod configreader;
mod modules;

fn activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
}

pub fn main() {}
