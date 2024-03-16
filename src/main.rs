extern crate speedy2d;

use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use crate::draw_tree::{draw_thin_line, draw_tree};
use crate::tree_struct::{Tree, TreeE};

mod tree_struct;
mod draw_tree;

fn main() {
    println!("Hello, world!");
    let win = Window::new_centered("test", (640, 640)).unwrap();
    win.run_loop(MyHandler{ tree: tree_struct::test() });
}

struct MyHandler{
    tree: Tree,
}

impl WindowHandler for MyHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        draw_thin_line(Vec2::ZERO, 2.0, Vec2::new(240.0, 540.0), 78.0, Color::RED, graphics);
        self.tree.update();
        draw_tree(&self.tree, Vec2::new(320.0, 640.0), graphics);
        helper.request_redraw();
    }
}