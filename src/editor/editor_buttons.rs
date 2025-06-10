use macroquad::prelude::draw_text;
use macroquad::prelude::is_key_down;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::is_key_released;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::prelude::measure_text;
use macroquad::prelude::mouse_position;
use macroquad::prelude::KeyCode;
use macroquad::prelude::MouseButton;
use macroquad::prelude::TextDimensions;
use macroquad::prelude::Vec2;
use macroquad::prelude::DARKGRAY;
use macroquad::prelude::GRAY;
use macroquad::prelude::GREEN;
use macroquad::prelude::LIGHTGRAY;

use super::EditorButton;
use super::EditorElements;
use super::EditorState;

pub const BUTTON_SIZE: f32 = 21.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorButtons {
    Undo,
    Redo,
    Help,
    Grid,
    Snap,
    Color,
    // Thickness,
    // Zoom,
    // ZoomIn,
    // ZoonOut,
    Line,
    // Arc,
    // Poly,
    // PolyLine,
    Circle,
    // CircleLine,
    Ellipse,
    // EllipseLine,
    Rectangle,
    // RectangleLine,
    Triangle,
    Hexagon,
}

impl EditorButtons {
    pub fn text(&self) -> &str {
        match self {
            EditorButtons::Undo => "UNDO",
            EditorButtons::Redo => "REDO",
            EditorButtons::Help => "HELP",
            EditorButtons::Grid => "GRID",
            EditorButtons::Snap => "SNAP",
            EditorButtons::Color => "COLOR",
            // EditorButtons::ZoomIn => "ZOOM_IN",
            // EditorButtons::ZoomOut => "ZOOM_OUT",
            EditorButtons::Line => "LINE",
            // EditorButtons::Arc => "ARC",
            // EditorButtons::Poly => "POLY",
            EditorButtons::Circle => "CIRCLE",
            EditorButtons::Ellipse => "ELLIPSE",
            EditorButtons::Rectangle => "RECTANGLE",
            EditorButtons::Triangle => "TRIANGLE",
            EditorButtons::Hexagon => "HEXAGON",
        }
    }
    pub fn dimensions(&self) -> TextDimensions {
        let text = self.text();
        measure_text(text, None, BUTTON_SIZE as u16, 1.0)
    }
    pub fn draw(state: &mut EditorState) {
        let position: Vec2 = mouse_position().into();
        let buttons: Vec<EditorButton> = EditorButton::list();

        for i in buttons {
            let text = i.button.text();
            let text_dimensions = i.button.dimensions();
            let is_position = position.x >= i.x
                && position.x <= i.x + text_dimensions.width
                && position.y >= i.y - text_dimensions.height
                && position.y <= i.y;
            let color = match i.button {
                EditorButtons::Undo => {
                    if state.stack_undo.is_empty() {
                        DARKGRAY
                    } else if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
                EditorButtons::Redo => {
                    if state.stack_redo.is_empty() {
                        DARKGRAY
                    } else if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
                EditorButtons::Help => {
                    if is_position || state.help {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                EditorButtons::Snap => {
                    if is_position || state.snap {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                EditorButtons::Grid => {
                    if is_position || state.grid >= 1 {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                EditorButtons::Circle
                | EditorButtons::Ellipse
                | EditorButtons::Line
                // | EditorButtons::Arc
                // | EditorButtons::Poly
                | EditorButtons::Rectangle
                | EditorButtons::Triangle
                | EditorButtons::Hexagon => {
                    if is_position || i.button == EditorButtons::from(state.element) && state.draw {
                        GREEN
                    } else {
                        GRAY
                    }
                }
                _ => {
                    if is_position {
                        LIGHTGRAY
                    } else {
                        GRAY
                    }
                }
            };
            draw_text(text, i.x, i.y, i.size, color);
        }
    }
    pub fn actions(state: &mut EditorState) {
        if is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftSuper) {
            state.undo();
        }
        if is_key_pressed(KeyCode::Y) && is_key_down(KeyCode::LeftSuper) {
            state.redo();
        }

        if is_key_pressed(KeyCode::S) && is_key_down(KeyCode::LeftSuper) {
            state.snap = !state.snap;
        }
        if is_key_pressed(KeyCode::G) && is_key_down(KeyCode::LeftSuper) {
            if state.grid > 2 {
                state.grid = 0;
            } else {
                state.grid += 1;
            }
        }

        if is_key_pressed(KeyCode::Key1) && is_key_down(KeyCode::LeftSuper) {
            state.element = EditorElements::Line;
        }

        if is_key_down(KeyCode::H) {
            state.help = true;
        }
        if is_key_released(KeyCode::H) {
            state.help = false;
        }
        if is_key_pressed(KeyCode::E) {
            state.export();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(button) = EditorButton::find().take() {
                if [
                    EditorButtons::Ellipse,
                    EditorButtons::Line,
                    EditorButtons::Triangle,
                    EditorButtons::Rectangle,
                    EditorButtons::Circle,
                    EditorButtons::Hexagon,
                ]
                .contains(&button.button)
                {
                    if state.button == Some(button.button) {
                        state.draw = !state.draw;
                    } else {
                        state.draw = true;
                    }
                }
                match button.button {
                    EditorButtons::Undo => {
                        state.button = Some(EditorButtons::Undo);
                        state.undo();
                    }
                    EditorButtons::Redo => {
                        state.button = Some(EditorButtons::Redo);
                        state.redo();
                    }
                    EditorButtons::Help => {
                        state.button = Some(EditorButtons::Help);
                        state.help = !state.help;
                    }
                    // EditorButtons::Arc => {
                    //     // state.button = Some(EditorButtons::Arc);
                    //     // state.element = EditorElements::Arc;
                    // }
                    // EditorButtons::Poly => {
                    //     // state.button = Some(EditorButtons::Poly);
                    //     // state.element = EditorElements::Poly;
                    // }
                    EditorButtons::Ellipse => {
                        state.button = Some(EditorButtons::Ellipse);
                        state.element = EditorElements::Ellipse;
                    }
                    EditorButtons::Rectangle => {
                        state.button = Some(EditorButtons::Rectangle);
                        state.element = EditorElements::Rectangle;
                    }
                    EditorButtons::Triangle => {
                        state.button = Some(EditorButtons::Triangle);
                        state.element = EditorElements::Triangle;
                    }
                    EditorButtons::Hexagon => {
                        state.button = Some(EditorButtons::Hexagon);
                        state.element = EditorElements::Hexagon;
                    }
                    EditorButtons::Line => {
                        state.button = Some(EditorButtons::Line);
                        state.element = EditorElements::Line;
                    }
                    EditorButtons::Circle => {
                        state.button = Some(EditorButtons::Circle);
                        state.element = EditorElements::Circle;
                        // state.element_lines = !state.element_lines;
                    }
                    // Some(EditorButtons::CircleLine) => {
                    //     self.state.button = Some(EditorButtons::CircleLine);
                    //     self.state.element_shape = EditorElements::CircleLine;
                    // }
                    EditorButtons::Grid => {
                        state.button = Some(EditorButtons::Grid);
                        if state.grid > 2 {
                            state.grid = 0;
                        } else {
                            state.grid += 1;
                        }
                    }
                    EditorButtons::Snap => {
                        state.button = Some(EditorButtons::Snap);
                        state.snap = !state.snap;
                    }
                    EditorButtons::Color => {
                        // state.button = Some(EditorButtons::Grid);
                        // if state.grid > 2 {
                        //     state.grid = 0;
                        // } else {
                        //     state.grid += 1;
                        // }
                    }
                }
            }
        }
    }
}
