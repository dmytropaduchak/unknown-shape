use macroquad::prelude::mouse_position;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::Vec2;

use super::EditorButtons;
use super::BUTTON_SIZE;

#[derive(Clone, Debug)]
pub struct EditorButton {
    pub button: EditorButtons,
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl EditorButton {
    pub fn new(button: EditorButtons, x: f32, y: f32, size: f32) -> Self {
        Self { button, x, y, size }
    }
    pub fn find() -> Option<EditorButton> {
        let position: Vec2 = mouse_position().into();
        EditorButton::list()
            .iter()
            .find(|i| {
                let text_dimensions = i.button.dimensions();
                position.x >= i.x
                    && position.x <= i.x + text_dimensions.width
                    && position.y >= i.y - text_dimensions.height
                    && position.y <= i.y
            })
            .cloned()
    }
    pub fn list() -> Vec<EditorButton> {
        let width = screen_width();
        let height = screen_height();

        let left_top_buttons: Vec<EditorButton> = [
            EditorButtons::Grid,
            EditorButtons::Snap,
            EditorButtons::Redo,
            EditorButtons::Undo,
        ]
        .iter()
        .rev()
        .scan((10.0, 20.0), |(x, y), &button| {
            let dimensions = button.dimensions();
            let text_x = *x;
            *x += dimensions.width + 10.0;
            Some(EditorButton::new(button, text_x, *y, BUTTON_SIZE))
        })
        .collect();

        let left_bottom_buttons: Vec<EditorButton> = [
            EditorButtons::Line,
            // EditorButtons::Arc,
            // EditorButtons::Poly,
            EditorButtons::Circle,
            EditorButtons::Ellipse,
            EditorButtons::Rectangle,
            EditorButtons::Triangle,
            EditorButtons::Hexagon,
        ]
        .iter()
        .rev()
        .scan((10.0, height - (BUTTON_SIZE / 2.0)), |(x, y), &button| {
            let dimensions = button.dimensions();
            let text_x = *x;
            *x += dimensions.width + 10.0;
            Some(EditorButton::new(button, text_x, *y, BUTTON_SIZE))
        })
        .collect();

        let right_top_buttons: Vec<EditorButton> = [EditorButtons::Help]
            .iter()
            .rev()
            .scan((width, 20.0), |(x, y), &button| {
                let dimensions = button.dimensions();
                *x -= dimensions.width + 10.0;
                Some(EditorButton::new(button, *x, *y, BUTTON_SIZE))
            })
            .collect();

        let right_bottom_buttons: Vec<EditorButton> = [EditorButtons::Color]
            .iter()
            .rev()
            .scan((width, height - 10.0), |(x, y), &button| {
                let dimensions = button.dimensions();
                *x -= dimensions.width + 10.0;
                Some(EditorButton::new(button, *x, *y, BUTTON_SIZE))
            })
            .collect();

        [
            left_top_buttons,
            left_bottom_buttons,
            right_top_buttons,
            right_bottom_buttons,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
