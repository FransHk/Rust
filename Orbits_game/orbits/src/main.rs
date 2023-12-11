use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::WindowSettings;

// Quick set of (unoptimised) array operations
// used in this celestial body simulation
mod utils;
use utils::array_logic::{self as al, dot_product};

// Some constants used throughout the code
type Colour = [f32; 4];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GRAV_CONST: f64 = 10.0;

/// This object represents a celestial body along
/// with its properties like pos, vel and acceleration
struct Planet {
    id: i8,
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
    /// Construct new celestial body
    // fn new() -> Planet {
    //     Planet {
    //     position: [100.0, 100.0],
    //     size: [40.0, 40.0],
    //     colour: BLACK,
    //     velocity: [184.0, -242.0],
    //     acceleration: [-20.0, 20.0],
    //     id: 0,
    //     mass: 10.0,
    // }}

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
        //println!("{:?}", pos);
        graphics::Rectangle::new(self.colour).draw(pos, &c.draw_state, c.transform, g);
    }

    /// Simply checks for border collisions, turns
    /// body around on collision (bouncy balls)
    /// TODO consider correcting position to prevent clipping
    fn check_collision(&mut self, bounds: f64) {
        if (self.position[0] + self.size[0] >= 500.0) {
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if (self.position[0] <= 0.0) {
            self.velocity[0] *= -1.0;
            self.acceleration[0] *= -1.0;
        }
        if (self.position[1] + self.size[1] >= 500.0) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
        if (self.position[1] <= 0.0) {
            self.velocity[1] *= -1.0;
            self.acceleration[1] *= -1.0;
        }
    }
}

fn create_planets(amt: u32) -> Vec<Planet> {
    //(Planet, Vec<Planet>) {
    let mut other_planets: Vec<Planet> = Vec::<Planet>::new();
    // let planet = Planet {
    //     id: 0,
    //     colour: WHITE,
    //     position: [250.0, 150.0],
    //     velocity: [90.0, 40.0],
    //     acceleration: [0.0, 0.0],
    //     size: [10.0, 10.0],
    //     mass: 3.0,
    // };

    other_planets.push(Planet {
        id: 0,
        position: [200.0, 200.0],
        size: [50.0, 50.0],
        colour: WHITE,
        velocity: [0.0, 0.0],
        acceleration: [0.0, 0.0],
        mass: 450.0,
    });
    other_planets.push(Planet {
        id: 1,
        position: [200.0, 200.0],
        size: [50.0, 50.0],
        colour: WHITE,
        velocity: [0.0, 0.0],
        acceleration: [0.0, 0.0],
        mass: 450.0,
    });
    other_planets
}

fn main() {
    //let mut planet = Planet::new();
    //let (mut plan, mut other_plan) = create_planets(0);
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
            // player_planet.update(&args);
            for planet in planets.iter_mut() {
                planet.update(&args); // pass update args for 'dt' value to scale movement
            }
        }

        for i in 0..planets.len() {
            for j in 0..planets.len() {
                let grav = grav_force(&planets[i], &planets[j]);
                planets.apply_gravity(grav);
            }
        }

        // perform gravitational
    }
}

pub trait CelestialBody {
    fn mass(&self) -> f64;
    fn pos(&self) -> [f64; 2];
}

pub fn grav_force<CelestialBody>(mass1: &CelestialBody, mass2: &CelestialBody) -> [f64; 2] {
    let dist = al::subtract_arrays(mass1.pos(), mass2.pos());
    let sqr_dist = al::dot_product(dist, dist);
    let force_dir = al::normalise_vector(dist);
    let force = al::scalar_mult(force_dir, GRAV_CONST * -1.0 * mass2.mass());
    let force = al::scalar_mult(force, 1.0 / sqr_dist);

    force
    // println!("grav force: {:?}", force);
    //al::add_arrays(mass1.velocity, force)
    //println!("Planet {} interacting with planet {}", self.id, acting_force.id);
}
