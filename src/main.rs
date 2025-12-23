use iced::{
    Subscription, Task,
    time::{self, Duration},
    widget::{button, column, text},
    theme::Style
};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, event}
use iced_layershell::{application,
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

    iced::application("", Bar::update, Bar::view)
        .subscription(Bar::subscription)
        .settings(Settings {
                    layer_settings: LayerShellSettings {
                        size: Some((0, 400)),
                        exclusive_zone: 400,
                        anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
                        start_mode,
                        ..Default::default()
                    },
                    ..Default::default()
                })
        .run_with(Bar::new)
}

#[derive(Debug, Clone)]
enum Message {
    Update,
}

#[derive(Default)]
struct Bar {
    seconds: u32,
}

impl Bar {
    fn new() -> (Self, Task<Message>) {
        (Self { seconds: 0 }, Task::none())
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Update => update_mod(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![text(self.seconds)].into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }
}
