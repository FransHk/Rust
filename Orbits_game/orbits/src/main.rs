extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.   
}
pub struct Ball {
    rotation: f64,  // Rotation for the square.
    speed_x: f64, // x-axis movement for square
    pos_x: f64,
    pos_y: f64,
    size: f64,
    colour: [f32; 4],
}

impl Ball {
    fn init(&mut self) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        self.colour = RED;
    }

    fn draw(&mut self, mut gl: GlGraphics, args: &RenderArgs) {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            clear(self.colour, gl);    
        });
    }
}


    
impl App {
    fn render(&mut self, args: &RenderArgs, obj: &Ball) {
        obj.draw(self.gl, args);
    }

    
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        
    };

    let ball: Ball = Ball{
        rotation: 0.0,
        speed_x: 0.5,
        pos_x: 10.0,
        pos_y: 10.0,
        size: 50.0,
        colour: [1.0, 0.0, 0.0, 1.0]
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &ball);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args, ball);
        // }
    }
}