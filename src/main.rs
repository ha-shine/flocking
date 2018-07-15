extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate flocking;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use flocking::App;

fn main() {
    let opengl = OpenGL::V3_2;

    let height: u32 = 600;
    let width: u32 = 600;
    let count: u32 = 80;
    let ups: u64 = 120;

    let mut window: Window = WindowSettings::new("flocking", [width, height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl), width, height, count, ups);

    let mut settings = EventSettings::new();
    settings.ups = ups;

    let mut events = Events::new(settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}