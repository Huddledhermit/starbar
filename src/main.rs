use iced::theme;
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

fn init_bar() -> bar {
    let init_config = configreader::read_config();
    bar {
        config: init_config,
        modules: init_config.modules,
    }
}

fn update_mod() {
    println!("updated")
}

fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();
    let mut newbar = init_bar();
    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }

    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };
    build_pattern::application(window::default, namespace, update, view)
        .style(style)
        .subscription(subscription)
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
        .run()
}



struct bar {
    config: configreader::Cfg,
    modules: Vec<String>,
}
#[derive(Default)]
struct window{
    id: i32
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

fn update(bar:&mut window ,message: Message) -> Command<Message> {
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

fn view(bar: &window) -> Element<Message> {
    column![button("test").on_press(Message::Update)].into()
}

fn subscription(_: &window) -> iced::Subscription<Message> {
    event::listen().map(Message::IcedEvent);
    time::every(Duration::from_secs(1)).map(|_| Message::Update)
}

fn style(_counter: &window, theme: &iced::Theme) -> iced::theme::Style {
    theme::Style {
        background_color: Color::TRANSPARENT,
        text_color: theme.palette().text,
    }
}
