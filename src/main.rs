use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

struct MyWindowHandler {}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        helper.request_redraw();
    }
}

fn main() {
    println!("Hello, world!");
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    window.run_loop(MyWindowHandler {});
}
