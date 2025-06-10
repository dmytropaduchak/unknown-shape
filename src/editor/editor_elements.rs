use macroquad::prelude::draw_circle_lines;
use macroquad::prelude::draw_line;
use macroquad::prelude::is_mouse_button_pressed;
use macroquad::prelude::is_mouse_button_released;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::MouseButton;
use macroquad::prelude::Vec2;
use macroquad::prelude::DARKGRAY;
use macroquad::prelude::YELLOW;

// use crate::studio::EditorShapes;
const SIZE_RESTRICTION: f32 = 10.0;

use super::EditorButtons;
use super::EditorElement;
use super::EditorState;
use super::EditorValues;
use super::SIZE_POINT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorElements {
    // Arc,
    Line,
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

impl EditorElements {
    fn element(state: &mut EditorState, current: Vec2, position: Vec2) -> EditorElement {
        let element = state.element;
        let element_color = state.element_color;
        let element_value = match element {
            EditorElements::Hexagon => {
                let radius = current.distance(position);
                let center = current;
                let vertical = current.x < position.x;
                EditorValues::Hexagon {
                    center,
                    radius,
                    vertical,
                }
            }
            EditorElements::Circle => {
                let radius = current.distance(position);
                let center = current;
                EditorValues::Circle { center, radius }
            }
            EditorElements::Ellipse => {
                let width = (position.x - current.x).abs();
                let height = (position.y - current.y).abs();
                let rotation = 0.0;
                let center = current;
                EditorValues::Ellipse {
                    center,
                    height,
                    width,
                    rotation,
                }
            }
            EditorElements::Line => {
                let point_a = current;
                let point_b = position;
                let thickness = state.element_thickness;
                EditorValues::Line {
                    point_a,
                    point_b,
                    thickness,
                }
            }
            EditorElements::Rectangle => {
                let x = current.x.min(position.x);
                let y = current.y.min(position.y);
                let point = Vec2::new(x, y);
                let width = (position.x - current.x).abs();
                let height = (position.y - current.y).abs();
                let rotation = 0.0;
                EditorValues::Rectangle {
                    point,
                    width,
                    height,
                    rotation,
                }
            }
            EditorElements::Triangle => {
                let point_a = current;
                let point_b = position;
                let point_c = Vec2::new(current.x, current.y * 0.5);
                EditorValues::Triangle {
                    point_a,
                    point_b,
                    point_c,
                }
            }
        };
        EditorElement::new(element_value, element_color)
    }

    pub fn draw(state: &mut EditorState) {
        let width = screen_width();
        let height = screen_height();
        let position = state.position();

        if state.draw && !state.drag {
            if let Some(current) = state.current {
                let element = EditorElements::element(state, current, position);
                let element_color = DARKGRAY;
                element.draw(Some(element_color));
            }
        }
        if !state.draw && state.drag {
            if let Some(element) = state.stack.iter_mut().find(|i| match i.value {
                EditorValues::Circle { center, radius } => {
                    return position.distance(center) <= radius;
                }
                EditorValues::Hexagon { center, radius, .. } => {
                    return position.distance(center) <= radius;
                }
                EditorValues::Rectangle {
                    point,
                    width,
                    height,
                    rotation,
                } => {
                    let p = position - point;
                    let rotation_sin = rotation.sin();
                    let rotation_cos = rotation.cos();

                    let x = p.x * rotation_cos + p.y * rotation_sin;
                    let y = -p.x * rotation_sin + p.y * rotation_cos;

                    return x >= -width / 2.0
                        && x <= width / 2.0
                        && y >= -height / 2.0
                        && y <= height / 2.0;
                }
                _ => false,
            }) {
                match element.value {
                    EditorValues::Circle { ref mut center, .. } => {
                        *center = position - state.drag_offset.unwrap_or(position);
                    }
                    EditorValues::Hexagon { ref mut center, .. } => {
                        *center = position - state.drag_offset.unwrap_or(position);
                    }
                    EditorValues::Rectangle { ref mut point, .. } => {
                        *point = position - state.drag_offset.unwrap_or(position);
                    }
                    _ => {}
                }
            }
        }

        for element in state.stack.iter() {
            element.draw(None);
        }

        let color = YELLOW.with_alpha(0.2);
        for element in state.stack.iter() {
            match element.value {
                EditorValues::Line {
                    point_a, point_b, ..
                } => {
                    if position.distance(point_a) <= SIZE_POINT
                        || position.distance(point_b) <= SIZE_POINT
                    {
                        draw_circle_lines(point_a.x, point_a.y, SIZE_POINT, 1.0, color);
                        draw_circle_lines(point_b.x, point_b.y, SIZE_POINT, 1.0, color);
                    }
                    if state.snap {
                        if (position.x - point_a.x).abs() < SIZE_POINT {
                            draw_line(point_a.x, 0.0, point_a.x, height, 1.0, color);
                        }
                        if (position.y - point_a.y).abs() < SIZE_POINT {
                            draw_line(0.0, point_a.y, width, point_a.y, 1.0, color);
                        }
                        if (position.x - point_b.x).abs() < SIZE_POINT {
                            draw_line(point_b.x, 0.0, point_b.x, height, 1.0, color);
                        }
                        if (position.y - point_b.y).abs() < SIZE_POINT {
                            draw_line(0.0, point_b.y, width, point_b.y, 1.0, color);
                        }
                    }
                }
                EditorValues::Circle { center, radius } => {
                    let point1 = Vec2::new(center.x, center.y + radius);
                    let point2 = Vec2::new(center.x, center.y - radius);
                    let point3 = Vec2::new(center.x + radius, center.y);
                    let point4 = Vec2::new(center.x - radius, center.y);
                    if position.distance(center) <= SIZE_POINT
                        || position.distance(point1) <= SIZE_POINT
                        || position.distance(point2) <= SIZE_POINT
                        || position.distance(point3) <= SIZE_POINT
                        || position.distance(point4) <= SIZE_POINT
                    {
                        draw_circle_lines(center.x, center.y, SIZE_POINT, 1.0, color);
                    }

                    if state.snap {
                        if (position.x - center.x).abs() < SIZE_POINT {
                            draw_line(center.x, 0.0, center.x, height, 1.0, color);
                        }
                        if (position.y - center.y).abs() < SIZE_POINT {
                            draw_line(0.0, center.y, width, center.y, 1.0, color);
                        }
                        if (position.y - point1.y).abs() < SIZE_POINT {
                            draw_line(0.0, point1.y, width, point1.y, 1.0, color);
                        }
                        if (position.y - point2.y).abs() < SIZE_POINT {
                            draw_line(0.0, point2.y, width, point2.y, 1.0, color);
                        }
                        if (position.x - point3.x).abs() < SIZE_POINT {
                            draw_line(point3.x, 0.0, point3.x, height, 1.0, color);
                        }
                        if (position.x - point4.x).abs() < SIZE_POINT {
                            draw_line(point4.x, 0.0, point4.x, height, 1.0, color);
                        }
                    }
                }
                EditorValues::Rectangle {
                    width,
                    height,
                    rotation,
                    point,
                } => {
                    let hw = width / 2.0;
                    let hh = height / 2.0;

                    let mut corners = [
                        Vec2::new(-hw, -hh),
                        Vec2::new(hw, -hh),
                        Vec2::new(hw, hh),
                        Vec2::new(-hw, hh),
                    ];

                    for corner in &mut corners {
                        *corner = Vec2::new(
                            corner.x * rotation.cos() - corner.y * rotation.sin(),
                            corner.x * rotation.sin() + corner.y * rotation.cos(),
                        ) + point;
                    }

                    let mut highlight = false;
                    for corner in &corners {
                        if position.distance(*corner) <= SIZE_POINT {
                            highlight = true;
                            break;
                        }
                    }
                    if position.distance(point) <= SIZE_POINT {
                        highlight = true;
                    }

                    if highlight {
                        draw_circle_lines(point.x, point.y, SIZE_POINT, 1.0, color);
                    }

                    if state.snap {
                        if (position.x - point.x).abs() < SIZE_POINT {
                            draw_line(point.x, 0.0, point.x, height, 1.0, color);
                        }
                        if (position.y - point.y).abs() < SIZE_POINT {
                            draw_line(0.0, point.y, width, point.y, 1.0, color);
                        }
                        for corner in &corners {
                            if (position.x - corner.x).abs() < SIZE_POINT {
                                draw_line(corner.x, 0.0, corner.x, height, 1.0, color);
                            }
                            if (position.y - corner.y).abs() < SIZE_POINT {
                                draw_line(0.0, corner.y, width, corner.y, 1.0, color);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    pub fn actions(state: &mut EditorState) {
        let position = state.position();

        if is_mouse_button_pressed(MouseButton::Left) && state.draw {
            state.current = Some(position);
        }

        if is_mouse_button_released(MouseButton::Left) && state.draw {
            if let Some(current) = state.current.take() {
                if current.distance(position) > SIZE_RESTRICTION {
                    let element = EditorElements::element(state, current, position);
                    state.save();
                    state.stack.push(element);
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) && !state.draw {
            if let Some(element) = state.stack.iter().find(|i| match i.value {
                EditorValues::Circle { center, radius } => {
                    return position.distance(center) <= radius;
                }
                EditorValues::Hexagon { center, radius, .. } => {
                    return position.distance(center) <= radius;
                }
                EditorValues::Rectangle {
                    point,
                    width,
                    height,
                    rotation,
                } => {
                    let p = position - point;
                    let rotation_sin = rotation.sin();
                    let rotation_cos = rotation.cos();

                    let x = p.x * rotation_cos + p.y * rotation_sin;
                    let y = -p.x * rotation_sin + p.y * rotation_cos;

                    return x >= -width / 2.0
                        && x <= width / 2.0
                        && y >= -height / 2.0
                        && y <= height / 2.0;
                }
                _ => false,
            }) {
                match element.value {
                    EditorValues::Circle { center, .. } => {
                        state.drag_offset = Some(position - center);
                        state.drag = true;
                    }
                    EditorValues::Hexagon { center, .. } => {
                        state.drag_offset = Some(position - center);
                        state.drag = true;
                    }
                    EditorValues::Rectangle { point, .. } => {
                        state.drag_offset = Some(position - point);
                        state.drag = true;
                    }
                    _ => {}
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) && !state.draw {
            state.drag = false;
        }
    }
}

impl From<EditorElements> for EditorButtons {
    fn from(i: EditorElements) -> Self {
        match i {
            // EditorElements::Arc => EditorButtons::Arc,
            EditorElements::Line => EditorButtons::Line,
            // EditorElements::Poly => EditorButtons::Poly,
            EditorElements::Circle => EditorButtons::Circle,
            // EditorShapes::CircleLine => EditorButtons::Circle,
            EditorElements::Ellipse => EditorButtons::Ellipse,
            EditorElements::Rectangle => EditorButtons::Rectangle,
            EditorElements::Triangle => EditorButtons::Triangle,
            EditorElements::Hexagon => EditorButtons::Hexagon,
        }
    }
}
