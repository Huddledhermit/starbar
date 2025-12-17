use iced::{widget::{button, column, text},
time::{self, Duration},
Subscription,
};
mod configreader;
mod modules;

fn update(clock: &modules::Clock){
    clock.Update_Time();
}

fn main() -> iced::Result {

    iced::run("", Bar::update, Bar::view)
}

#[derive(Debug, Clone)]
enum Message {
    Update
}

#[derive(Default)]
struct Bar {
    counter: usize,
}

impl Bar {
    fn update(&mut self, message: Message) {
        match Message {
            Message::Update => update
        }
    }

    fn view(&self) -> iced::Element<Message> {

        column![
            text(&self.counter),
            button("Increase").on_press(Message::ButtonPressed),
        ]
        .into()
    }

    fn Subscription(&self){
        time::every(Duration::from_secs(1)).map(|_| Message::Update)
    }

    }
}
