use super::body_config::*;
use crate::utils::array_logic as al;
use crate::utils::colour::Colour;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::UpdateArgs;
use rand::Rng;
use rand_distr::{Distribution, Normal};

impl Planet {
    fn init_planet(planet_const: &PlanetConfig) -> ([f64; 2], [f64; 2], f64) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(planet_const.lower_pos_bound..planet_const.upper_pos_bound);
        let y = rng.gen_range(planet_const.lower_pos_bound..planet_const.upper_pos_bound);
        let vel_x = rng.gen_range(-planet_const.velocity_bound..planet_const.velocity_bound);
        let vel_y = rng.gen_range(-planet_const.velocity_bound..planet_const.velocity_bound);
        let pos: [f64; 2] = [x, y];
        let vel: [f64; 2] = [vel_x, vel_y];
        let normal = Normal::new(planet_const.mass_mean, planet_const.mass_std).unwrap();
        let mass = normal.sample(&mut rng);

        (pos, vel, mass)
    }

    pub fn new(planet_const: &PlanetConfig, id: u32, colour: Colour) -> Planet {
        let (pos, vel, mass) = Planet::init_planet(planet_const);

        Planet {
            id,
            colour,
            position: pos,
            velocity: vel,
            acceleration: [0.0, 0.0],
            mass,
            size: [
                mass * planet_const.mass_to_size,
                mass * planet_const.mass_to_size,
            ],
            config: planet_const.clone(),
        }
    }

    // Reset the planet by obtaining a new
    // set of pos, vel, mass, size bearings
    fn reset_planet(&mut self) {
        let (pos, vel, mass) = Planet::init_planet(&self.config);
        self.position = pos;
        self.velocity = vel;
        self.mass = mass;
        self.size = [
            mass * self.config.mass_to_size,
            mass * self.config.mass_to_size,
        ];
    }

    // Add 2D force to this body
    pub fn add_force(&mut self, force: [f64; 2]) {
        self.velocity = al::add_arrays(self.velocity, force);
    }

    /// Perform step-wise updates to velocity and position
    pub fn update(&mut self, args: &UpdateArgs) {
        // scale by deltatime (e.g. move velocity[0] p/sec on x, velocity[1] p/sec on y
        // this makes movement movement frame-independent
        let scaled_acc = al::scalar_mult(self.acceleration, args.dt); // scale by deltatime
        self.velocity = al::add_arrays(self.velocity, scaled_acc);
        let scaled_vel = al::scalar_mult(self.velocity, args.dt);
        self.position = al::add_arrays(self.position, scaled_vel);
    }

    /// Accept created graphical context and GL object,
    /// draw this planet to that graphical context  
    pub fn draw(&self, c: graphics::Context, g: &mut GlGraphics) {
        let pos: [f64; 4] = [
            self.position[0],
            self.position[1],
            self.size[0],
            self.size[1],
        ];
        graphics::Rectangle::new(self.colour).draw(pos, &c.draw_state, c.transform, g);
    }
    pub fn check_dist_from_centre(&mut self, centre: [f64; 2]) {
        let dist = al::subtract_arrays(self.pos(), centre);
        let dist_len = al::get_length(dist);

        if (dist_len > 700.0) {
            self.reset_planet();
        }
    }

    /// Simply checks for border collisions, turns
    /// body around on collision (bouncy balls)
    pub fn check_collision(&mut self, bounds: f64) {
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

/// This object represents a celestial body along
/// with its properties like pos, vel and acceleration
#[derive(Debug)]
pub struct Planet {
    id: u32,
    colour: Colour,
    position: [f64; 2],
    velocity: [f64; 2],
    acceleration: [f64; 2],
    size: [f64; 2],
    mass: f64,
    config: PlanetConfig,
}

impl CelestialBody for Planet {
    fn mass(&self) -> f64 {
        self.mass
    }
    fn pos(&self) -> [f64; 2] {
        self.position
    }
}
