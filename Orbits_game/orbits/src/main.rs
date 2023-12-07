use glutin_window::GlutinWindow;
use piston::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use opengl_graphics::{GlGraphics, OpenGL};

// Quick set of (unoptimised) array operations
// used in this celestial body simulation
mod utils;

// Some constants used throughout the code
type Colour = [f32; 4];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];


/// This object represents a celestial body along
/// with its properties like pos, vel and acceleration
struct Planet {
    colour: Colour,
    position: [f64; 2],
    velocity: [f64; 2],
    acceleration: [f64; 2],
    size: [f64; 2],
}

impl Planet {
    /// Construct new celestial body
    fn new() -> Planet {
        Planet {
        position: [100.0, 100.0],
        size: [40.0, 40.0],
        colour: BLACK,
        velocity: [184.0, -242.0],
        acceleration: [-20.0, 20.0],
    }}

    /// Perform step-wise updates to velocity and position
    fn update(&mut self, args: &UpdateArgs){
        // Scale acceleration and velocity with deltatime
        // to make speed and acceleration frame-independent, 
        // then apply to the position of the planet
        let scaled_acc = utils::array_logic::scalar_mult(self.acceleration, args.dt);
        self.velocity = utils::array_logic::add_arrays(self.velocity, scaled_acc);
        let scaled_vel = utils::array_logic::scalar_mult(self.velocity, args.dt);
        self.position = utils::array_logic::add_arrays(self.position, scaled_vel);
    }

    /// Accept created graphical context and GL object,
    /// draw this planet to that graphical context  
    fn draw(&self, c: graphics::Context, g: &mut GlGraphics)
    {
         let pos: [f64; 4] = [self.position[0], self.position[1], self.size[0], self.size[1]];
         //println!("{:?}", pos);
         graphics::Rectangle::new(self.colour).draw(pos, 
                                                    &c.draw_state,
                                                    c.transform,
                                                    g);
    }

    /// Simply checks for border collisions, turns
    /// body around on collision (bouncy balls)
    fn check_collision(&mut self, bounds: f64){
        if(self.position[0] + self.size[0] >= 500.0){
            println!("Bounce to left");
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if(self.position[0] <= 0.0) {
            println!("Bounce to right");
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if(self.position[1] + self.size[1] >= 500.0) {
            println!("Bounce up");
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
        if(self.position[1] <= 0.0) {
            println!("Bounce down");
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
    }
}

fn main() {
    let mut planet = Planet::new();   
    let bounds: f64 = 512.0; // essentially the window size

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Window", [bounds; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new()); 

    // Game loop. First, render every object (planet),
    // then, update each planet's position and check 
    // for collisions.
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
                let tile = &mut planet; // Create reference to the planet object, we do not want ownership or a copy
                graphics::clear(BLUE, g); 
                tile.draw(c, g); // draw tile 
                });            }
            
        if let Some(args) = e.update_args() {
                let tile = &mut planet; // Create reference to the planet object, we do not want ownership or a copy
                tile.update(&args); // Update pos, vel
                tile.check_collision(bounds); // Check if border is exceeded, if so, flip
            }
    }
}