use iced;
mod configreader;
mod modules;

fn main() -> iced::Result {}
enum Message {}
#[derive(Default)]
struct starbar {}
impl starbar {
    fn update(&mut self, message: Message) {
        match message {}
    }
    fn view(&self) {
        column![].into
    }
}
