use iced::{widget::{button, text_input, Column, Row, Text}, Alignment::Center, Element};

#[derive(Default)]
pub struct Visualizer {
    number: String,
    count: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    ContentChanged(String),
}

impl Visualizer {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.count += 1;
            },
            Message::ContentChanged(content) => {
                self.number = content;
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let visualize_button = button("Press me!").on_press(Message::ButtonPressed);
        let number_input = text_input("Enter your on number or select from the below list", &self.number)
            .on_input(Message::ContentChanged);
        // let number_selector =combo_box(state, placeholder, selection, on_selected)

        let column = Column::new()
            .align_x(iced::Center)
            .spacing(20)
            .push(number_input)
            .push(Text::new(format!("Button pressed: {}", self.count)))
            .push(visualize_button);

        Row::new()
            .align_y(Center)
            .spacing(20)
            .push(column)
            .into()
    }
}