use iced::{
    Subscription, Task,
    time::{self, Duration},
    widget::{button, column, text},
};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, event}
use iced_layershell::{build_pattern,
    reexport::Anchor,
settings::{LayerShellSettings, StartMode, Settings},
to_layer_message };
mod configreader;
mod modules;

fn update_mod() {
    println!("updated")
}

fn main() -> Result<(), iced_layershell::Error>  {
    let config = configreader::read_config;

    let args: Vec<String> = std::env::args().collect();

        let mut binded_output_name = None;
        if args.len() >= 2 {
            binded_output_name = Some(args[1].to_string())
        }

        let start_mode = match binded_output_name {
            Some(output) => StartMode::TargetScreen(output),
            None => StartMode::Active,

            build_pattern::application(namespace,)

        };

fn namespace() -> String {
    String::from("test window")
}
#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Update,
}

fn update(message: Message) {
    match message {
        Message::Update => update_mod(),
        }
}

fn view() -> iced::Element<Message> {

}

fn subscription() -> iced::Subscription<Message> {
    time::every(Duration::from_secs(1)).map(|_| Message::Update)
}
