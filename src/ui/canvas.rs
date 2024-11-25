// use iced::widget::canvas;
use iced::Color;
use std::collections::HashMap;

pub struct Canvas {
    colors: Vec<String>,
    degrees: HashMap<String, i16>,
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
        let colors = vec![
            "D9CD85".to_string(),
            "C5B884".to_string(),
            "CC8C8D".to_string(),
            "9E5872".to_string(),
            "935771".to_string(),
            "997897".to_string(),
            "868291".to_string(),
            "6A878B".to_string(),
            "87AC9B".to_string(),
            "B9BEA7".to_string()
        ];
        
        let mut degrees = HashMap::<String, i16>::new();

        let slice_degrees = 360 / 10;
        let mut counter = 0;
        while counter < 10 {
            degrees.insert(counter.to_string(), slice_degrees * counter);
            counter = counter + 1;
        }

        println!("Colors: {:?}, degrees: {:?}", colors.clone(), degrees.clone());

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