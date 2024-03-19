use std::f32::consts::PI;
use crate::tree_struct;

use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use speedy2d::shape::Polygon;
use speedy2d::window::VirtualKeyCode::V;
use crate::tree_struct::TreeE;

fn rotate_90(v: Vec2) -> Vec2{
    Vec2{
        x: -v.y,
        y: v.x,
    }
}
pub fn draw_thin_line(begin_pos: Vec2, begin_thin: f32, end_pos: Vec2, end_thin: f32, color: Color, graphics2d: &mut Graphics2D){
    let s = end_pos - begin_pos;
    if let Some(normal) = s.normalize(){
        let rot_norm = rotate_90(normal);
        let poly = Polygon::new(&[
            rot_norm * begin_thin * 0.5 + begin_pos,
            rot_norm * begin_thin * -0.5 + begin_pos,
            rot_norm * end_thin * -0.5 + end_pos,
            rot_norm * end_thin * 0.5 + end_pos
        ]);
        graphics2d.draw_polygon(&poly, Vec2::ZERO, color);
    }
}

pub fn get_begin_point(center: Vec2, radius: f32, angle: f32, thin: f32) -> Vec2{
    let normal = Vec2::new(angle.cos(), angle.sin());
    let offset = (radius.powi(2) - (thin * 0.5).powi(2)).sqrt();
    center + normal * offset
}

fn rotate(v: Vec2, a: f32) -> Vec2{
    let (sin, cos) = (a.sin(), a.cos());
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

fn sheet_poly(angle: f32) -> Polygon{
    let mut shape: Vec<Vec2> = Vec::new();
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;
    let scale = 6.0;
    while x < 2.8{
        shape.push(Vec2::new(x * scale, y * scale));
        x += 0.2;
        y = 2.0 - ((x - 1.0).powi(2) + 1.0).sqrt();
    }
    let len = shape.len();
    for i in 1..len{
        shape.push(Vec2::new(shape[len-i].x, -shape[len-i].y))
    }
    for p in &mut shape{
        *p = rotate(*p, angle);
    }
    Polygon::new(shape.as_slice())
}

pub fn draw_sheet(pos: Vec2, angle: f32, graphics2d: &mut Graphics2D){
    let poly = sheet_poly(angle);
    let color =Color::from_rgb(0.0, (-angle).sin().abs() * 0.5 + 0.5, 0.0);
    graphics2d.draw_polygon(&poly, pos, color);
    graphics2d.draw_circle(pos, 2.0, color);
}

#[derive(Copy, Clone)]
struct DrawerState{
    pos: Vec2,
    angle: f32,
    last_thin: Option<f32>,
}
pub fn draw_tree(tree: &tree_struct::Tree, pos: Vec2, graphics2d: &mut Graphics2D){
    let mut stack: Vec<DrawerState> = Vec::new();
    let mut state = DrawerState{
        pos,
        angle: -0.5 * PI,
        last_thin: None,
    };
    for el in &tree.l_system{
        match el {
            TreeE::Segment {
                length, thickness, angle, is_trunk, ..
            } => {
                state.angle += angle;
                let mut point1 = state.pos;
                if let Some(last_thin) = state.last_thin{
                    point1 = get_begin_point(point1, last_thin * 0.5, *angle, *thickness);
                }
                let normal = Vec2::new(state.angle.cos(), state.angle.sin());
                let point2 = state.pos + normal * *length;
                draw_thin_line(point1, *thickness, point2, *thickness, Color::BLACK, graphics2d);
                graphics2d.draw_circle(point2, *thickness * 0.5, Color::BLACK);
                if !*is_trunk{
                    draw_sheet(point2, state.angle, graphics2d);
                }
                state.pos = point2;
            },
            TreeE::OpBracket => {
                stack.push(state.clone());
            }
            TreeE::ClBracket => {
                if let Some(old_state) = stack.pop(){
                    state = old_state;
                }
            }
            TreeE::Flower {
                ..
            } => {
                //draw_sheet(state.pos, 0.0, graphics2d);
            }
            _ => {}
        }
    }
}