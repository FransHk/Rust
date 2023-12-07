use glutin_window::GlutinWindow;
use graphics::Transformed;
use piston::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use opengl_graphics::{GlGraphics, OpenGL};
mod utils; // some array logic utilities

type Colour = [f32; 4];
const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];

struct Planet {
    colour: Colour,
    position: [f64; 2],
    velocity: [f64; 2],
    acceleration: [f64; 2],
    size: [f64; 2],
}

impl Planet {
    fn new() -> Planet {
        Planet {
        position: [0.0, 0.0],
        size: [60.0, 60.0],
        colour: BLACK,
        velocity: [150.0, 110.0],
        acceleration: [1.0, 1.0],
    }}

    fn update(&mut self, args: &UpdateArgs){
        let scaled_acc = utils::array_logic::scalar_mult(self.acceleration, args.dt);
        let scaled_vel = utils::array_logic::scalar_mult(self.velocity, args.dt);
        self.velocity = utils::array_logic::add_arrays(self.velocity, scaled_acc);
        self.position = utils::array_logic::add_arrays(self.position, scaled_vel);
       // println!("{:?}", self.velocity);
        
    }
    
    fn draw(&self, c: graphics::Context, g: &mut GlGraphics)
    {
         let pos: [f64; 4] = [self.position[0], self.position[1], self.size[0], self.size[1]];
         graphics::Rectangle::new(self.colour).draw(pos, 
                                                    &c.draw_state,
                                                    c.transform,
                                                    g);
    }

    fn check_collision(&mut self, bounds: f64){
        if(self.position[0] + self.size[0] >= 500.0){
            println!("Bounce to right!");
            self.position[0] = 500.0 - self.size[0];
            self.velocity[0] *= -1.0;
        }
        if(self.position[0] + self.size[0] <= 0.0) {
            self.position[0] = 0.0 + self.size[0];
            self.velocity[0] *= -1.0;
        }
        if(self.position[1] + self.size[1] >= 500.0) {
            self.position[1] = 500.0 - self.size[1];
            self.velocity[1] *= -1.0;
        }
        if(self.position[1] - self.size[1] <= 0.0) {
            self.position[1] = 0.0 + self.size[1];
            self.velocity[1] *= -1.0;
        }
    }
}

fn main() {
    let mut planet = Planet::new();   
    let bounds: f64 = 512.0;

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Window", [bounds; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new()); 

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
                let tile = &mut planet; // Create reference to the planet object, we do not want ownership or a copy
                graphics::clear(WHITE, g); 
                tile.draw(c, g); // draw tile 
                });
            }
            
        if let Some(args) = e.update_args() {
                let tile = &mut planet; // Create reference to the planet object, we do not want ownership or a copy
                tile.check_collision(bounds);
                tile.update(&args); // Update pos, vel
            }
    }
}