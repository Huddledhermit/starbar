use iced::{Alignment, Color, Element, Event, Length, Task as Command, event};
use iced::{
    Subscription, Task,
    time::{self, Duration},
    widget::{Button, button, column, text},
};
use iced_layershell::build_pattern;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;
mod configreader;
mod modules;

fn update_mod() {
    println!("updated")
}

fn main() -> Result<(), iced_layershell::Error> {
    let active_bar = bar {
        config: configreader::read_config(),
    };

    let args: Vec<String> = std::env::args().collect();

    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }

    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };
    build_pattern::application(active_bar, namespace, update, view)
        .subscription(subscription)
        .settings(Settings {
            Layer_settings: LayerShellSettings { size: (0, 400) },
        })
        .run()
}

// #[derive(Default)]
struct bar {
    config: configreader::Cfg,
}

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
    Top,
    Bottom,
    Left,
    Right,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    IcedEvent(Event),
    Update,
    Direction(WindowDirection),
}

fn namespace() -> String {
    String::from("test_window")
}

fn update(counter: &mut bar, message: Message) -> Command<Message> {
    match message {
        Message::IcedEvent(event) => {
            println!("hello {event:?}");
            Command::none()
        }

        Message::Direction(direction) => match direction {
            WindowDirection::Left => Command::done(Message::AnchorSizeChange(
                Anchor::Left | Anchor::Top | Anchor::Bottom,
                (400, 0),
            )),
            WindowDirection::Right => Command::done(Message::AnchorSizeChange(
                Anchor::Right | Anchor::Top | Anchor::Bottom,
                (400, 0),
            )),
            WindowDirection::Bottom => Command::done(Message::AnchorSizeChange(
                Anchor::Bottom | Anchor::Left | Anchor::Right,
                (0, 400),
            )),
            WindowDirection::Top => Command::done(Message::AnchorSizeChange(
                Anchor::Top | Anchor::Left | Anchor::Right,
                (0, 400),
            )),
        },
        _ => unreachable!(),
    }
}

fn view() -> Element<Message> {
    column![button("test").on_press(Message::Update)].into()
}

fn subscription(_: &bar) -> iced::Subscription<Message> {
    event::listen().map(Message::IcedEvent);
    time::every(Duration::from_secs(1)).map(|_| Message::Update)
}
