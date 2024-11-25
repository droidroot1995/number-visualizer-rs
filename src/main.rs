pub mod ui;
use crate::ui::visualizer::Visualizer;
use iced;

fn main() -> iced::Result {
    iced::run("Number visualizer", Visualizer::update, Visualizer::view)
}