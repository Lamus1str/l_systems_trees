extern crate speedy2d;

use std::io::stdin;
use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use crate::draw_tree::{draw_thin_line, draw_tree};
use crate::tree_struct::{Tree, TreeE};

mod tree_struct;
mod draw_tree;

fn main() {
    let mut seed = [0_u8; 32];
    let mut input = String::new();
    stdin().read_line(&mut input);
    let bytes = input.as_bytes();
    if bytes.len() < 32{
        for i in 0..32{
            seed[i] = rand::random();
        }
        println!("{}", String::from_utf8_lossy(&seed));
    }
    else {
        for i in 0..32{
            seed[i] = bytes[i];
        }
    }
    let win = Window::new_centered("Korega, reqiem, da", (640, 640)).unwrap();
    win.run_loop(MyHandler{ tree: tree_struct::Tree::new(seed), timer: 30.0 });
}

struct MyHandler{
    tree: Tree,
    timer: f32,

}

impl WindowHandler for MyHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if self.timer >= 0.0{
            self.timer -= 0.04;
            graphics.clear_screen(Color::WHITE);
            //draw_thin_line(Vec2::ZERO, 2.0, Vec2::new(240.0, 540.0), 78.0, Color::RED, graphics);
            self.tree.update();
            draw_tree(&self.tree, Vec2::new(320.0, 640.0), graphics);
            helper.request_redraw();
        }
    }
}