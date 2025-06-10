use macroquad::prelude::draw_line;
use macroquad::prelude::draw_rectangle_lines;
use macroquad::prelude::draw_text;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::GRAY;
use macroquad::prelude::LIGHTGRAY;
use macroquad::prelude::RED;
use macroquad::prelude::YELLOW;

use super::EditorState;
use super::DISPLAY_SIZE;
use super::DISPLAY_SIZE_HD;
use super::SIZE_GRID;

pub struct EditorHelps {}

impl EditorHelps {
    pub fn draw(state: &mut EditorState) {
        let width = screen_width();
        let height = screen_height();

        if state.grid > 0 {
            if state.grid >= 1 {
                let grid_color = GRAY.with_alpha(0.1);
                let grid_size = SIZE_GRID;
                for x in (0..=(width as i32 / grid_size as i32)).map(|i| i as f32 * grid_size) {
                    draw_line(x, 0.0, x, height, 1.0, grid_color);
                }
                for y in (0..=(height as i32 / grid_size as i32)).map(|i| i as f32 * grid_size) {
                    draw_line(0.0, y, width, y, 1.0, grid_color);
                }
            }

            if state.grid == 2 {
                let grid_color = GRAY.with_alpha(0.1);
                let grid_size = SIZE_GRID * 5.0;
                for x in (0..=(width as i32 / grid_size as i32)).map(|i| i as f32 * grid_size) {
                    draw_line(x, 0.0, x, height, 1.0, grid_color);
                }
                for y in (0..=(height as i32 / grid_size as i32)).map(|i| i as f32 * grid_size) {
                    draw_line(0.0, y, width, y, 1.0, grid_color);
                }
            }
        }

        let display_x = width / 2.0 - DISPLAY_SIZE.x / 2.0;
        let display_y = height / 2.0 - DISPLAY_SIZE.y / 2.0;
        let display_color = RED.with_alpha(0.3);
        draw_text(
            format!(
                "{}X{}",
                DISPLAY_SIZE.x.round() as u16,
                DISPLAY_SIZE_HD.y.round() as u16
            )
            .as_str(),
            display_x,
            display_y - 10.0,
            18.0,
            RED.with_alpha(0.5),
        );
        draw_rectangle_lines(
            display_x,
            display_y,
            DISPLAY_SIZE.x,
            DISPLAY_SIZE.y,
            2.0,
            display_color,
        );

        let display_x = width / 2.0 - DISPLAY_SIZE_HD.x / 2.0;
        let display_y = height / 2.0 - DISPLAY_SIZE_HD.y / 2.0;
        let display_color = RED.with_alpha(0.3);
        draw_text(
            format!(
                "{}X{}",
                DISPLAY_SIZE_HD.x.round() as u16,
                DISPLAY_SIZE_HD.y.round() as u16
            )
            .as_str(),
            display_x,
            display_y - 10.0,
            18.0,
            RED.with_alpha(0.5),
        );
        draw_rectangle_lines(
            display_x,
            display_y,
            DISPLAY_SIZE_HD.x,
            DISPLAY_SIZE_HD.y,
            2.0,
            display_color,
        );
    }
    pub fn actions(state: &mut EditorState) {
        if state.help {
            let help_items = [
                ("HELP", ""),
                ("[CMD+Z]", "Undo the last action"),
                ("[CMD+Y]", "Redo the undone action"),
                ("[CMD+S]", "Toggle snap mode, align to nearby points"),
                ("[CMD+G]", "Toggle background grid visibility"),
                ("[H]", "Show or hide this help overlay"),
            ];

            let text_size = 20.0;
            let spacing = 6.0;
            let line_height = text_size + spacing;
            let total_height = help_items.len() as f32 * line_height;

            let start_y = screen_height() / 2.0 - total_height / 2.0;
            let padding = 20.0;

            for (i, (shortcut, description)) in help_items.iter().enumerate() {
                let y = start_y + i as f32 * line_height;

                if description.is_empty() {
                    draw_text(shortcut, padding, y, text_size, YELLOW);
                } else {
                    draw_text(shortcut, padding, y, text_size, LIGHTGRAY);
                    draw_text(description, padding + 80.0, y, text_size, GRAY);
                }
            }
        }
    }
}
