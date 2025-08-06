use macroquad::prelude::draw_line;
use macroquad::prelude::mouse_position;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::Color;
use macroquad::prelude::DrawRectangleParams;
use macroquad::prelude::Vec2;
use macroquad::prelude::WHITE;
use macroquad::prelude::YELLOW;

use super::EditorButtons;
use super::EditorElement;
use super::EditorElements;
use super::EditorValues;

pub const DISPLAY_SIZE: Vec2 = Vec2::new(640.0, 480.0);
pub const DISPLAY_SIZE_HD: Vec2 = Vec2::new(1280.0, 720.0);

pub const STICKY: f32 = 10.0;
pub const STICKY_ELEMENT: f32 = 5.0;

pub const SIZE_GRID: f32 = 10.0;
pub const SIZE_POINT: f32 = 3.0;

#[derive(Debug, Clone)]
pub struct EditorState {
    // pub element: Option<EditorElement>,
    pub element: EditorElements,
    pub element_thickness: f32,
    pub element_color: Color,
    pub element_color_index: usize,
    // pub element_lines: bool,
    pub stack: Vec<EditorElement>,
    pub stack_undo: Vec<Vec<EditorElement>>,
    pub stack_redo: Vec<Vec<EditorElement>>,

    pub current: Option<Vec2>,

    pub button: Option<EditorButtons>,

    pub draw: bool,
    pub snap: bool,
    pub grid: u16,
    pub help: bool,

    pub drag: bool,
    pub drag_offset: Option<Vec2>,
    // pub position_cursor: Option<Vec2>,
    // pub button:
    // pub thickness: f32,
    // Position in the stack for the current line
    //
    // pub current_start: Option<Vec2>,
    // pub sticky_radius: f32,

    // pub dragging_point: Option<(usize, bool)>, // (line index, is_start)
    // pub drag_start_mouse_pos: Option<Vec2>,    // Mouse pos when drag started
    // pub drag_original_positions: Vec<(usize, bool, Vec2)>, // All matching points original positions

    // pub show_points: bool,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            stack_undo: Vec::new(),
            stack_redo: Vec::new(),
            // element: None,
            element: EditorElements::Line,
            element_thickness: 1.0,
            element_color: WHITE.with_alpha(0.5),
            element_color_index: 0,
            // element_lines: false,
            current: None,
            // select
            button: Some(EditorButtons::Line),

            draw: true,
            snap: true,
            grid: 2,
            help: false,

            drag: false,
            drag_offset: None,
            // cursor: None,
            // current_start: None,
            // sticky_radius: 10.0,

            // dragging_point: None,
            // drag_start_mouse_pos: None,
            // drag_original_positions: Vec::new(),

            // undo_stack: Vec::new(),
            // redo_stack: Vec::new(),

            // show_points: true,
        }
    }

    pub fn save(&mut self) {
        self.stack_undo.push(self.stack.clone());
        self.stack_redo.clear();
    }

    pub fn undo(&mut self) {
        if let Some(stack) = self.stack_undo.pop() {
            self.stack_redo.push(self.stack.clone());
            self.stack = stack;
        }
    }

    pub fn redo(&mut self) {
        if let Some(stack) = self.stack_redo.pop() {
            self.stack_undo.push(self.stack.clone());
            self.stack = stack;
        }
    }

    pub fn export(&self) {
        let mut content = String::new();

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for i in &self.stack {
            match i.value {
                EditorValues::Line {
                    point_a, point_b, ..
                } => {
                    min_x = min_x.min(point_a.x.min(point_b.x));
                    min_y = min_y.min(point_a.y.min(point_b.y));
                    max_x = max_x.max(point_a.x.max(point_b.x));
                    max_y = max_y.max(point_a.y.max(point_b.y));
                }
                EditorValues::Circle { center, radius }
                | EditorValues::CircleLine { center, radius } => {
                    min_x = min_x.min(center.x - radius);
                    min_y = min_y.min(center.y - radius);
                    max_x = max_x.max(center.x + radius);
                    max_y = max_y.max(center.y + radius);
                }
                EditorValues::Hexagon { center, radius, .. } => {
                    min_x = min_x.min(center.x - radius);
                    min_y = min_y.min(center.y - radius);
                    max_x = max_x.max(center.x + radius);
                    max_y = max_y.max(center.y + radius);
                }
                EditorValues::Ellipse {
                    center,
                    width,
                    height,
                    rotation,
                }
                | EditorValues::EllipseLine {
                    center,
                    width,
                    height,
                    rotation,
                } => {
                    let hw = width / 2.0;
                    let hh = height / 2.0;
                    let cos_r = rotation.cos();
                    let sin_r = rotation.sin();

                    let corners = [
                        Vec2::new(-hw, -hh),
                        Vec2::new(hw, -hh),
                        Vec2::new(hw, hh),
                        Vec2::new(-hw, hh),
                    ];

                    for corner in corners.iter() {
                        let rotated = Vec2::new(
                            corner.x * cos_r - corner.y * sin_r,
                            corner.x * sin_r + corner.y * cos_r,
                        );
                        let world = center + rotated;

                        min_x = min_x.min(world.x);
                        min_y = min_y.min(world.y);
                        max_x = max_x.max(world.x);
                        max_y = max_y.max(world.y);
                    }
                }
                EditorValues::Rectangle {
                    point,
                    width,
                    height,
                    rotation,
                } => {
                    if rotation == 0.0 {
                        // Simple case for non-rotated rectangles
                        min_x = min_x.min(point.x);
                        min_y = min_y.min(point.y);
                        max_x = max_x.max(point.x + width);
                        max_y = max_y.max(point.y + height);
                    } else {
                        // Handle rotated rectangles
                        let cos_r = rotation.cos();
                        let sin_r = rotation.sin();

                        let corners = [
                            Vec2::new(0.0, 0.0),
                            Vec2::new(width, 0.0),
                            Vec2::new(width, height),
                            Vec2::new(0.0, height),
                        ];

                        for corner in corners.iter() {
                            let rotated = Vec2::new(
                                corner.x * cos_r - corner.y * sin_r,
                                corner.x * sin_r + corner.y * cos_r,
                            );
                            let world = point + rotated;

                            min_x = min_x.min(world.x);
                            min_y = min_y.min(world.y);
                            max_x = max_x.max(world.x);
                            max_y = max_y.max(world.y);
                        }
                    }
                }
                EditorValues::Triangle {
                    point_a,
                    point_b,
                    point_c,
                } => {
                    min_x = min_x.min(point_a.x.min(point_b.x.min(point_c.x)));
                    min_y = min_y.min(point_a.y.min(point_b.y.min(point_c.y)));
                    max_x = max_x.max(point_a.x.max(point_b.x.max(point_c.x)));
                    max_y = max_y.max(point_a.y.max(point_b.y.max(point_c.y)));
                }
            }
        }
        let width = max_x - min_x;
        let height = max_y - min_y;

        content.push_str("fn draw(x: f32, y: f32) {\n");
        content.push_str(&format!(
            "   draw_rectangle_lines(x + {:.1}, y + {:.1}, {:.1}, {:.1}, 1.2, {:?});\n",
            0.0, 0.0, width, height, YELLOW
        ));

        for i in self.stack.iter() {
            let color = i.color;
            match i.value {
                EditorValues::Line {
                    point_a,
                    point_b,
                    thickness,
                } => {
                    content.push_str(&format!(
                        "   draw_line(x + {:.1}, y + {:.1}, x + {:.1}, y + {:.1}, {:.1}, {:?});\n",
                        point_a.x - min_x,
                        point_a.y - min_y,
                        point_b.x - min_x,
                        point_b.y - min_y,
                        thickness,
                        color,
                    ));
                }
                EditorValues::Circle { center, radius } => {
                    content.push_str(&format!(
                        "   draw_circle(x + {:.1}, y + {:.1}, {:.1}, {:?});\n",
                        center.x - min_x,
                        center.y - min_y,
                        radius,
                        color,
                    ));
                }
                EditorValues::CircleLine { center, radius } => {
                    content.push_str(&format!(
                        "   draw_circle_lines(x + {:.1}, y + {:.1}, {:.1}, 1.0, {:?});\n",
                        center.x - min_x,
                        center.y - min_y,
                        radius,
                        color,
                    ));
                }
                EditorValues::Ellipse {
                    center,
                    width,
                    height,
                    rotation,
                } => {
                    content.push_str(&format!(
                        "   draw_ellipse(x + {:.1}, y + {:.1}, {:.1}, {:.1}, {:.1}, {:?});\n",
                        center.x - min_x,
                        center.y - min_y,
                        width,
                        height,
                        rotation,
                        color,
                    ));
                }
                EditorValues::EllipseLine {
                    center,
                    width,
                    height,
                    rotation,
                } => {
                    content.push_str(&format!(
                        "   draw_ellipse_lines(x + {:.1}, y + {:.1}, {:.1}, {:.1}, {:.1}, 1.0, {:?});\n",
                        center.x - min_x,
                        center.y - min_y,
                        width,
                        height,
                        rotation,
                        color,
                    ));
                }
                EditorValues::Rectangle {
                    width,
                    height,
                    rotation,
                    point,
                } => {
                    let params = DrawRectangleParams {
                        offset: Vec2::new(0.0, 0.0),
                        rotation,
                        color,
                    };
                    content.push_str(&format!(
                        "   draw_rectangle_ex(x + {:.1}, y + {:.1}, {:.1}, {:.1}, {:?});\n",
                        point.x - min_x,
                        point.y - min_y,
                        width,
                        height,
                        params,
                    ));
                }
                EditorValues::Triangle {
                    point_a,
                    point_b,
                    point_c,
                } => {
                    content.push_str(&format!(
                        "   draw_triangle(Vec2::new(x + {:.1}, y + {:.1}), Vec2::new(x + {:.1}, y + {:.1}), Vec2::new(x + {:.1}, y + {:.1}), {:?});\n",
                        point_a.x - min_x,
                        point_a.y - min_y,
                        point_b.x - min_x,
                        point_b.y - min_y,
                        point_c.x - min_x,
                        point_c.y - min_y,
                        color,
                    ));
                }
                EditorValues::Hexagon {
                    center,
                    radius,
                    vertical,
                } => {
                    content.push_str(&format!(
                        "   draw_hexagon(x + {:.1}, y + {:.1}, {:.1}, 1.0, {:?}, {:?}, {:?});\n",
                        center.x - min_x,
                        center.y - min_y,
                        radius,
                        vertical,
                        color,
                        color,
                    ));
                }
            }
        }
        content.push_str("}\n");
        println!("\n{}", content);
    }
    pub fn position(&mut self) -> Vec2 {
        let position: Vec2 = mouse_position().into();

        if self.snap {
            let width = screen_width();
            let height = screen_height();

            let color = YELLOW.with_alpha(0.2);

            let mut position_snap = position;

            if self.grid > 0 {
                let nearest_x = (position_snap.x / SIZE_GRID).round() * SIZE_GRID;
                if (position_snap.x - nearest_x).abs() < STICKY {
                    position_snap.x = nearest_x;
                }
                let nearest_y = (position_snap.y / SIZE_GRID).round() * SIZE_GRID;
                if (position_snap.y - nearest_y).abs() < STICKY {
                    position_snap.y = nearest_y;
                }
            }

            let display = Vec2::new(
                width / 2.0 - DISPLAY_SIZE.x / 2.0,
                height / 2.0 - DISPLAY_SIZE.y / 2.0,
            );
            if (position.x - display.x).abs() < STICKY {
                position_snap.x = display.x;
                draw_line(position_snap.x, 0.0, position_snap.x, height, 1.0, color);
            }
            if (position.x - (display.x + DISPLAY_SIZE.x)).abs() < STICKY {
                position_snap.x = display.x + DISPLAY_SIZE.x;
                draw_line(position_snap.x, 0.0, position_snap.x, height, 1.0, color);
            }
            if (position.y - display.y).abs() < STICKY {
                position_snap.y = display.y;
                draw_line(0.0, position_snap.y, width, position_snap.y, 1.0, color);
            }
            if (position.y - (display.y + DISPLAY_SIZE.y)).abs() < STICKY {
                position_snap.y = display.y + DISPLAY_SIZE.y;
                draw_line(0.0, position_snap.y, width, position_snap.y, 1.0, color);
            }

            let display = Vec2::new(
                width / 2.0 - DISPLAY_SIZE_HD.x / 2.0,
                height / 2.0 - DISPLAY_SIZE_HD.y / 2.0,
            );
            if (position.x - display.x).abs() < STICKY {
                position_snap.x = display.x;
                draw_line(position_snap.x, 0.0, position_snap.x, height, 1.0, color);
            }
            if (position.x - (display.x + DISPLAY_SIZE_HD.x)).abs() < STICKY {
                position_snap.x = display.x + DISPLAY_SIZE_HD.x;
                draw_line(position_snap.x, 0.0, position_snap.x, height, 1.0, color);
            }
            if (position.y - display.y).abs() < STICKY {
                position_snap.y = display.y;
                draw_line(0.0, position_snap.y, width, position_snap.y, 1.0, color);
            }
            if (position.y - (display.y + DISPLAY_SIZE_HD.y)).abs() < STICKY {
                position_snap.y = display.y + DISPLAY_SIZE_HD.y;
                draw_line(0.0, position_snap.y, width, position_snap.y, 1.0, color);
            }

            if (position.x - 0.0).abs() < STICKY {
                position_snap.x = 0.0;
            }
            if (position.x - width).abs() < STICKY {
                position_snap.x = width;
            }
            if (position.y - 0.0).abs() < STICKY {
                position_snap.y = 0.0;
            }
            if (position.y - height).abs() < STICKY {
                position_snap.y = height;
            }

            for i in self.stack.iter() {
                match i.value {
                    EditorValues::Line {
                        point_a, point_b, ..
                    } => {
                        if position.distance(point_a) <= STICKY_ELEMENT {
                            position_snap = point_a;
                        }
                        if position.distance(point_b) <= STICKY_ELEMENT {
                            position_snap = point_b;
                        }

                        if (position.x - point_a.x).abs() < SIZE_POINT {
                            position_snap.x = point_a.x;
                        }
                        if (position.y - point_a.y).abs() < SIZE_POINT {
                            position_snap.y = point_a.y;
                        }

                        if (position.x - point_b.x).abs() < SIZE_POINT {
                            position_snap.x = point_b.x;
                        }
                        if (position.y - point_b.y).abs() < SIZE_POINT {
                            position_snap.y = point_b.y;
                        }
                    }
                    EditorValues::Circle { center, radius } => {
                        let point1 = Vec2::new(center.x, center.y + radius);
                        let point2 = Vec2::new(center.x, center.y - radius);
                        let point3 = Vec2::new(center.x + radius, center.y);
                        let point4 = Vec2::new(center.x - radius, center.y);
                        if position.distance(center) <= STICKY_ELEMENT {
                            position_snap = center;
                        }
                        if position.distance(point1) <= STICKY_ELEMENT {
                            position_snap = point1;
                        }
                        if position.distance(point2) <= STICKY_ELEMENT {
                            position_snap = point2;
                        }
                        if position.distance(point3) <= STICKY_ELEMENT {
                            position_snap = point3;
                        }
                        if position.distance(point4) <= STICKY_ELEMENT {
                            position_snap = point4;
                        }

                        if (position.x - center.x).abs() < SIZE_POINT {
                            position_snap.x = center.x;
                        }
                        if (position.y - center.y).abs() < SIZE_POINT {
                            position_snap.y = center.y;
                        }
                        if (position.y - point1.y).abs() < SIZE_POINT {
                            position_snap.y = point1.y;
                        }
                        if (position.y - point2.y).abs() < SIZE_POINT {
                            position_snap.y = point2.y;
                        }
                        if (position.x - point3.x).abs() < SIZE_POINT {
                            position_snap.x = point3.x;
                        }
                        if (position.x - point4.x).abs() < SIZE_POINT {
                            position_snap.x = point4.x;
                        }

                        let direction = (position - center).normalize();
                        let point = center + direction * radius;

                        if (position.x - point.x).abs() < SIZE_POINT {
                            position_snap.x = point.x;
                        }
                        if (position.y - point.y).abs() < SIZE_POINT {
                            position_snap.y = point.y;
                        }
                    }
                    _ => {
                        // TODO...
                    }
                }
            }

            position_snap
        } else {
            position
        }
    }
}
