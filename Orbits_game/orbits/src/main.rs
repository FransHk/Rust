use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::WindowSettings;
use rand::Rng;
// Quick set of (unoptimised) array operations
// used in this celestial body simulation
mod utils;
use utils::array_logic::{self as al};

// Some constants used throughout the code
type Colour = [f32; 4];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GRAV_CONST: f64 = 5.0;

/// This object represents a celestial body along
/// with its properties like pos, vel and acceleration
#[derive(Debug)]
struct Planet {
    id: u32,
    colour: Colour,
    position: [f64; 2],
    velocity: [f64; 2],
    acceleration: [f64; 2],
    size: [f64; 2],
    mass: f64,
}

impl CelestialBody for Planet {
    fn mass(&self) -> f64 {
        self.mass
    }
    fn pos(&self) -> [f64; 2] {
        self.position
    }
}

impl Planet {
    // TODO let it take a size enum (large, med, small) 
    // to initialise either (e.g. make it a multiplier for size, mass..)
    fn new(bounds: f64, id: u32, colour: Colour) -> Planet{
        let mut rng = rand::thread_rng();
        let lower_bound = 0.0;
        let upper_bound = bounds;
        let vel_bound = 10.0;
        let mass_lower = 0.1;
        let mass_upper = 4.0;

        let random_x = rng.gen_range(lower_bound..upper_bound);
        let random_y = rng.gen_range(lower_bound..upper_bound);
        let vel_x = rng.gen_range(-vel_bound..vel_bound);
        let vel_y = rng.gen_range(-vel_bound..vel_bound);
        let mass: f64 = rng.gen_range(mass_lower..mass_upper);

        let pos: [f64; 2] = [random_x, random_y];
        let vel: [f64; 2] = [vel_x, vel_y];

        Planet { 
            id: id,
            colour: colour,
            position: pos,
            velocity: vel, 
            acceleration: [0.0, 0.0], // TODO consider removing?
            size: [mass * 0.5, mass * 0.5], // TODO set this to self.ratio
            mass: mass,
        }
    }

    fn reset_pos(&self) {
        // TODO implement this method
    }

    fn add_force(&mut self, force: [f64; 2]) {
        self.velocity = al::add_arrays(self.velocity, force);
    }

    /// Perform step-wise updates to velocity and position
    fn update(&mut self, args: &UpdateArgs) {
        // scale by deltatime (e.g. move velocity[0] p/sec on x, velocity[1] p/sec on y
        // this makes movement movement frame-independent
        let scaled_acc = al::scalar_mult(self.acceleration, args.dt); // scale by deltatime
        self.velocity = al::add_arrays(self.velocity, scaled_acc);
        let scaled_vel = al::scalar_mult(self.velocity, args.dt);
        self.position = al::add_arrays(self.position, scaled_vel);
    }

    /// Accept created graphical context and GL object,
    /// draw this planet to that graphical context  
    fn draw(&self, c: graphics::Context, g: &mut GlGraphics) {
        let pos: [f64; 4] = [
            self.position[0],
            self.position[1],
            self.size[0],
            self.size[1],
        ];
        graphics::Rectangle::new(self.colour).draw(pos, &c.draw_state, c.transform, g);
    }
    fn check_dist_from_centre(&mut self, centre: [f64; 2]){
        let dist = al::subtract_arrays(self.pos(), centre);
        let dist_len= al::get_length(dist); 
        if(dist_len > 500.0){
        }
    }

    /// Simply checks for border collisions, turns
    /// body around on collision (bouncy balls)
    /// TODO consider correcting position to prevent clipping
    fn check_collision(&mut self, bounds: f64) {
        if (self.position[0] + self.size[0] >= bounds) {
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if (self.position[0] <= 0.0) {
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if (self.position[1] + self.size[1] >= bounds) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
        if (self.position[1] <= 0.0) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
    }
    
}

fn create_planets(amt: u32, bounds: f64) -> Vec<Planet> {
    //(Planet, Vec<Planet>) {
    let mut planets = Vec::<Planet>::new();
    for i in 0..amt {
        planets.push(Planet::new(bounds, i, WHITE));
    }
    

    planets
}

fn main() {
    //let mut planet = Planet::new();
    //let (mut plan, mut other_plan) = create_planets(0);
    let bounds: f64 = 1024.0; // essentially the window size
    let centre: [f64; 2] = [bounds * 0.5, bounds * 0.5];
    let mut planets = create_planets(40, bounds);

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Window", [bounds; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    // Game loop. First, render every object (planet),
    // then, update each planet's position and check
    // for collisions.
    while let Some(e) = events.next(&mut window) {
        // let player_planet: &mut Planet = &mut plan;
        // let other_planets = &mut other_plan;

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c: graphics::Context, g: &mut GlGraphics| {
                graphics::clear(BLACK, g);

                // player_planet.draw(c, g);
                for planet in planets.iter() {
                    planet.draw(c, g);
                }
            });
        }

        if let Some(args) = e.update_args() {
            for planet in planets.iter_mut() {
                planet.update(&args); // pass update args for 'dt' value to scale movement
                //planet.check_dist_from_centre(centre); // println!("{:?}", planet.pos());
                planet.check_collision(bounds);
            }
            for i in 0..planets.len() {
                for j in 0..planets.len() {
                    if (i != j) {
                        let (grav, is_colliding) = grav_force(&planets[i], &planets[j]);
                        if (!is_colliding) {
                            planets[i].add_force(grav);
                        } else {
                        // todo handle collision
                        }
                    }
                }
            }
        }

        // perform gravitational
    }
}

pub trait CelestialBody {
    fn mass(&self) -> f64;
    fn pos(&self) -> [f64; 2];
}

pub fn grav_force<C: CelestialBody>(mass1: &C, mass2: &C) -> ([f64; 2], bool) {
    let dist = al::subtract_arrays(mass1.pos(), mass2.pos());
    let dist_length = al::get_length(dist);
    let sqr_dist = al::dot_product(dist, dist);
    let force_dir = al::normalise_vector(dist);
    let force = al::scalar_mult(force_dir, GRAV_CONST * -1.0 * mass2.mass());
    let force = al::scalar_mult(force, 1.0 / sqr_dist);
    let colliding = dist_length <= 0.3;
    (force, colliding)
}
