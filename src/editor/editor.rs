use macroquad::prelude::clear_background;
use macroquad::prelude::next_frame;
use macroquad::prelude::Color;
use macroquad::prelude::BLACK;

use super::EditorButtons;
use super::EditorElements;
use super::EditorHelps;
use super::EditorState;

pub struct Editor {
    color: Color,
    state: EditorState,
}

impl Editor {
    pub fn new() -> Self {
        let color = BLACK;
        let state = EditorState::new();
        Editor { color, state }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.color.with_alpha(0.5));

            EditorButtons::actions(&mut self.state);
            EditorButtons::draw(&mut self.state);

            EditorElements::actions(&mut self.state);
            EditorElements::draw(&mut self.state);

            EditorHelps::actions(&mut self.state);
            EditorHelps::draw(&mut self.state);

            next_frame().await;
        }
    }
}
