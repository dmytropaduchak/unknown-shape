use macroquad::prelude::draw_circle;
use macroquad::prelude::draw_ellipse;
use macroquad::prelude::draw_hexagon;
use macroquad::prelude::draw_line;
use macroquad::prelude::draw_rectangle_ex;
use macroquad::prelude::draw_triangle;
use macroquad::prelude::Color;
use macroquad::prelude::DrawRectangleParams;
use macroquad::prelude::Vec2;
use macroquad::shapes::draw_circle_lines;
use macroquad::shapes::draw_ellipse_lines;

#[derive(Debug, Clone, Copy)]
pub enum EditorValues {
    Line {
        point_a: Vec2,
        point_b: Vec2,
        thickness: f32,
    },
    Circle {
        center: Vec2,
        radius: f32,
    },
    CircleLine {
        center: Vec2,
        radius: f32,
    },
    Ellipse {
        center: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
    },
    EllipseLine {
        center: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
    },
    Rectangle {
        point: Vec2,
        width: f32,
        height: f32,
        rotation: f32,
    },
    Triangle {
        point_a: Vec2,
        point_b: Vec2,
        point_c: Vec2,
    },
    Hexagon {
        center: Vec2,
        radius: f32,
        vertical: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct EditorElement {
    pub color: Color,
    pub value: EditorValues,
    // use super::EditorShapes;

    // pub struct EditorElement {
    //     color: Color,
    //     shape: EditorShapes,
    //     element: EditorElements,
    //     // pub id: usize,
    //     // pub a: Vec2,
    //     // pub b: Vec2,
    //     // // pub c: Option<Vec2>,
    //     // // pub rotation: Option<Vec2>,
    //     // pub color: Color,
    //     // pub shape: EditorElements,
    //     // pub props: EditorElementProps,
    //     // pub sides: Option<usize>,
    // }
}

impl EditorElement {
    pub fn new(value: EditorValues, color: Color) -> Self {
        Self { color, value }
    }
    pub fn draw(&self, color: Option<Color>) {
        let color = color.unwrap_or(self.color);
        match self.value {
            EditorValues::Line {
                point_a,
                point_b,
                thickness,
            } => {
                let a_x = point_a.x;
                let a_y = point_a.y;
                let b_x = point_b.x;
                let b_y = point_b.y;
                draw_line(a_x, a_y, b_x, b_y, thickness, color);
            }
            // EditorValues::Poly => {
            //     let radius = current.distance(position);
            //     let sides = (radius / 10.0).clamp(3.0, 12.0) as u8;
            //     let dx = position.x - current.x;
            //     let dy = position.y - current.y;
            //     let rotation = dy.atan2(dx);
            //     draw_poly(current.x, current.y, sides, radius, rotation, color);
            // }
            EditorValues::Circle { center, radius } => {
                let x = center.x;
                let y = center.y;
                draw_circle(x, y, radius, color);
            }
            EditorValues::CircleLine { center, radius } => {
                let x = center.x;
                let y = center.y;
                draw_circle_lines(x, y, radius, 1.0, color);
            }
            // EditorElements::Arc => {
            //     let radius = current.distance(position);
            //     let sides = (radius / 4.0).clamp(12.0, 64.0) as u8;
            //     let dx = position.x - current.x;
            //     let dy = position.y - current.y;
            //     let angle = dy.atan2(dx).to_degrees();
            //     let arc = 180.0;
            //     draw_arc(current.x, current.y, sides, arc, angle, 1.0, radius, color);
            // }
            EditorValues::Ellipse {
                center,
                width,
                height,
                rotation,
            } => {
                let x = center.x;
                let y = center.y;
                draw_ellipse(x, y, width, height, rotation, color);
            }
            EditorValues::EllipseLine {
                center,
                width,
                height,
                rotation,
            } => {
                let x = center.x;
                let y = center.y;
                draw_ellipse_lines(x, y, width, height, rotation, 1.0, color);
            }
            EditorValues::Rectangle {
                point,
                width,
                height,
                rotation,
            } => {
                let x = point.x;
                let y = point.y;
                let offset = Vec2::new(0.0, 0.0);
                draw_rectangle_ex(
                    x,
                    y,
                    width,
                    height,
                    DrawRectangleParams {
                        color,
                        rotation,
                        offset,
                    },
                );
            }
            EditorValues::Triangle {
                point_a,
                point_b,
                point_c,
            } => {
                draw_triangle(point_a, point_b, point_c, color);
            }
            EditorValues::Hexagon {
                center,
                radius,
                vertical,
            } => {
                let x = center.x;
                let y = center.y;
                draw_hexagon(x, y, radius, 1.0, vertical, color, color);
            }
        }
    }
}
