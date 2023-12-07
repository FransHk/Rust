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

struct Tile {
    colour: Colour,
    position: [f64; 2],
    velocity: [f64; 2],
    acceleration: [f64; 2],
}

impl Tile {
    fn new() -> Tile {
        Tile {
        position: [10.0, 20.0],
        colour: BLACK,
        velocity: [15.0, 0.0],
        acceleration: [1.0, 1.0],
    }}

    fn update(&mut self, args: &UpdateArgs){
        let scaled_acc = utils::array_logic::scalar_mult(self.acceleration, args.dt);
        let scaled_vel = utils::array_logic::scalar_mult(self.velocity, args.dt);
        self.velocity = utils::array_logic::add_arrays(self.velocity, scaled_acc);
        self.position = utils::array_logic::add_arrays(self.position, scaled_vel);
        println!("{:?}", self.velocity);
        
    }
    
    fn draw(&self, c: graphics::Context, g: &mut GlGraphics)
    {
         let pos: [f64; 4] = [self.position[0], self.position[1], 60.0, 60.0];
         graphics::Rectangle::new(BLACK).draw(pos, 
                                                    &c.draw_state,
                                                    c.transform,
                                                    g);
    }

    fn check_collision(){

    }
}
fn main() {
    let mut first_tile = Tile::new();
    
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Window", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new()); 

    while let Some(e) = events.next(&mut window) {

    if let Some(r) = e.render_args() {
        gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
            let tile = &mut first_tile; // Create reference to the first_tile object, we do not want ownership or a copy
            graphics::clear(BLUE, g); 
            tile.draw(c, g); 
            });
        }
        
    if let Some(args) = e.update_args() {
            let tile = &mut first_tile; // Create reference to the first_tile object, we do not want ownership or a copy
            tile.update(&args);
        }
    }
}



