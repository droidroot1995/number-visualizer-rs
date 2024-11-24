use iced::{widget::button, widget::Column, widget::Text};

fn main() -> iced::Result {
    iced::run("Number visualizer", Visualizer::update, Visualizer::view)
}

#[derive(Default)]
struct Visualizer {
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Visualizer {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.count += 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let button = button("Press me!").on_press(Message::ButtonPressed);

        Column::new()
            .align_x(iced::Center)
            .spacing(20)
            .push(Text::new(format!("Button pressed: {}", self.count)))
            .push(button)
            .into()
    }
}