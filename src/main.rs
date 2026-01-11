use gio::prelude::*;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
mod configreader;
mod modules;

fn activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::new(application);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.auto_excusive_zone_enable();
    window.set_margin(Edge::Left,40);
    window.set_margin(Edge::Right,40);
    window.set_margin(Edge::Top,40);
    let anchors = [
        (Edge::Left, true),
        (Edge::Right,true),
        (Edge::Top,true),
        (Edge::Bottom,false),

    ];

    for (anchor,state) in anchors{
        window.set_anchor(anchor,state);
    }

    let label = gtk::Label::new(Some(""));
       label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
       window.set_child(Some(&label));
       window.show()

}

pub fn main() {

    let application=gtk4::application::new(Some("sh.wmww.gtk-layer-example"), Default::default());
    application.connect_activate(|app|{
        activate(app);
    });

    application.run()
}
