use glutin_window::GlutinWindow;
use piston::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use opengl_graphics::{GlGraphics, OpenGL};

// Quick set of (unoptimised) array operations
// used in this celestial body simulation
mod utils;
use utils::array_logic as al;

// Some constants used throughout the code
type Colour = [f32; 4];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GRAV_CONST: f64 = 4.0;


/// This object represents a celestial body along
/// with its properties like pos, vel and acceleration
struct Planet {
    id: i8,
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
        id: 0,
    }}

    /// Perform step-wise updates to velocity and position
    fn update(&mut self, args: &UpdateArgs){
        // scale by deltatime (e.g. move velocity[0] p/sec on x, velocity[1] p/sec on y
        // this makes movement movement frame-independent
        let scaled_acc = al::scalar_mult(self.acceleration, args.dt); // scale by deltatime
        self.velocity = al::add_arrays(self.velocity, scaled_acc); 
        let scaled_vel = al::scalar_mult(self.velocity, args.dt);         
        self.position = al::add_arrays(self.position, scaled_vel);
    }
    //     TODO IMPLEMENT FULL SET OF LOGIC (what DOES forceDir * scalar do?)
    //     for coord in coords:
    //     diff = coord - attractor.coords # Get difference
    //     sqrDist = np.dot(diff, diff) # square magnitude of the distance (v * v = v^{2})
    //     forceDir = diff / np.linalg.norm(diff) # normalise the diff by dividing by norm (the magnitude of the vector)
    //     force = (forceDir * -1 * GRAV_CONST * attractor.mass) / sqrDist
    //     print(coord, force)
    //     new_coord = coord + force
    //     new_coords.append(new_coord)
    //     print("Transformed {} to {}".format(coord, new_coord))
    // return np.array(new_coords)
    
    fn grav_force(&mut self, acting_force: &Planet) {
        let dist = al::subtract_arrays(self.position, acting_force.position);
        let sqr_dist = al::dot_product(dist, dist);
        let force_dir = al::normalise_vector(dist);
        let force = al::scalar_mult(force_dir, GRAV_CONST * 10.0 * -1.0);
        let force = al::scalar_mult(force, 1.0/sqr_dist);
        self.velocity = al::add_arrays(self.velocity, force);
        // TODO IMPLEMENT ARRAY DIVISIONS
       // println!("{:?}", normalised); 
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
    /// TODO consider correcting position to prevent clipping
    fn check_collision(&mut self, bounds: f64){
        if(self.position[0] + self.size[0] >= 500.0){
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if(self.position[0] <= 0.0) {
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if(self.position[1] + self.size[1] >= 500.0) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
        if(self.position[1] <= 0.0) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
    }
}
fn create_planets(amt: u32) -> Vec<Planet> {
    let mut planets: Vec<Planet> = Vec::<Planet>::new();
    planets.push(Planet{
        id: 1,
        position: [200.0, 200.0],
        size: [20.0, 20.0],
        colour: WHITE,
        velocity: [0.0, 0.0],
        acceleration: [0.0, 0.0],
    });
    planets.push(Planet{
        id: 1,
        position: [200.0, 100.0],
        size: [20.0, 20.0],
        colour: WHITE,
        velocity: [0.0, 0.0],
        acceleration: [0.0, 0.0],
    });

    planets
}

fn main() {
    //let mut planet = Planet::new();   
    let mut planets = create_planets(0);
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
        let planets = &mut planets;
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
                graphics::clear(BLUE, g); 
                for planet in planets.iter() {
                    planet.draw(c, g);
                }
                });            }
            
        if let Some(args) = e.update_args() {
                for planet in planets.iter_mut() {
                    planet.update(&args); // pass update args for 'dt' value to scale movement
                }
                let planets_slice = planets.as_mut_slice();
                    for i in 0..planets_slice.len() {
                        let (left, right) = planets_slice.split_at_mut(i+1);
                        let planet = &mut left[i];
                        for other_planet in right.iter() {
                            planet.g    rav_force(other_planet);
                        }
                }
                // for planet in planets.iter_mut(){
                //     for other_planet in planets.iter(){
                //         if(planet.id != other_planet.id){
                //             planet.grav_force(&other_planet);
                //         }
                //     }
                // }
                // perform gravitational 
                    
        }
    }
}
