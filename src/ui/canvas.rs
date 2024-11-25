// use iced::widget::canvas;
use std::collections::HashMap;

pub struct Canvas {
    colors: HashMap<String, String>,
    degrees: HashMap<String, i8>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    ContentChanged(String),
}

impl Canvas {
    fn update(&mut self, _message: Message) {
    }

    // fn view(&self) -> Element<Message> {
// 
    // }
}

impl Default for Canvas {
    fn default() -> Self {
        let colors = HashMap::<String, String>::new();
        let degrees = HashMap::<String, i8>::new();

        Self {
            colors: colors,
            degrees: degrees,
        }

    }
}

/*impl<Message> canvas::Program<Message> for Canvas {
    type State = ();

    fn draw(&self){
    }
}*/