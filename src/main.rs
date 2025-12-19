use iced::{
    Subscription,
    Task,
    time::{self, Duration},
    widget::{button, column, text},
};
mod configreader;
mod modules;

fn update_mod() {
    println!("updated")
}

fn main() -> iced::Result {
    iced::application("", Bar::update, Bar::view)
        .subscription(Bar::subscription)
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
