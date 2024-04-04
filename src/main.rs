use iced::widget::*;
use iced::{Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result {
    println!("Hello, world!");

    Counter::run(iced::Settings::default())
}

#[derive(Default)]
struct Counter{
    value: i32
}

#[derive(Copy, Debug, Clone)]
pub enum Message{
    Increment,
    Decrement,
}

impl Sandbox for Counter{

    type Message = Message;

    fn new() -> Self{
        Self { value: 0 }
    }

    fn title(&self) -> String{
        String::from("Counter")
    }

    fn view(&self) -> Element<Message>{
        crate::column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message){
        match message{
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
}
