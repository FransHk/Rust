use super::body_config::*;
use crate::utils::array_logic::{self as al, Length};
use crate::utils::colour::Colour;
use bevy::prelude::*;
use rand::Rng;
use rand_distr::{Distribution, Normal};

impl Planet {
    pub fn new(planet_const: &PlanetConfig, id: u32) -> Planet {
        let (pos, vel, mass) = Planet::configure_planet(planet_const);

        Planet {
            id,
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

    fn configure_planet(planet_const: &PlanetConfig) -> ([f32; 2], [f32; 2], f32) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(planet_const.lower_pos_bound..planet_const.upper_pos_bound);
        let y = rng.gen_range(planet_const.lower_pos_bound..planet_const.upper_pos_bound);
        let vel_x = rng.gen_range(-planet_const.velocity_bound..planet_const.velocity_bound);
        let vel_y = rng.gen_range(-planet_const.velocity_bound..planet_const.velocity_bound);
        let pos: [f32; 2] = [x, y];
        let vel: [f32; 2] = [vel_x, vel_y];
        let normal = Normal::new(planet_const.mass_mean, planet_const.mass_std).unwrap();
        let mass = normal.sample(&mut rng);

        (pos, vel, mass)
    }
    // Reset the planet by obtaining a new
    // set of pos, vel, mass, size bearings
    fn reset_planet(&mut self) {
        let (pos, vel, mass) = Planet::configure_planet(&self.config);
        self.position = pos;
        self.velocity = vel;
    }

    /// Adds a 2-dimensional force to the body,
    /// it is scaled by the body's mass before being
    /// applied
    pub fn add_force(&mut self, force: [f32; 2]) {
        let scaled_force = al::scalar_mult(force, 1.0 / self.mass); // i.e. force/self.mass
        self.velocity = al::add_arrays(self.velocity, scaled_force);
    }

    // /// Perform step-wise updates to velocity and position
    pub fn update(&mut self, dt: f32) {
        // scale by deltatime (e.g. move velocity[0] p/sec on x, velocity[1] p/sec on y
        // this makes movement movement frame-independent
        let scaled_acc = al::scalar_mult(self.acceleration, dt); // scale by deltatime
        self.velocity = al::add_arrays(self.velocity, scaled_acc);
        let scaled_vel = al::scalar_mult(self.velocity, dt);
        self.position = al::add_arrays(self.position, scaled_vel);
    }

    /// Asserts distance from center, if out bounds,
    /// reset planet's pos, vel, etc.
    pub fn check_dist_from_centre(&mut self, centre: [f32; 2]) {
        let dist = al::subtract_arrays(self.position, centre);
        let dist_len = dist.get_length();

        if (dist_len > 700.0) {
            self.reset_planet();
        }
    }

    /// Checks for border collisions, turns
    /// body around on collision
    pub fn check_collision(&mut self, bounds: f32) {
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

#[derive(Component, Clone)]
pub struct Planet {
    pub id: u32,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub acceleration: [f32; 2],
    pub size: [f32; 2],
    pub mass: f32,
    config: PlanetConfig,
}

// impl CelestialBody for Planet {
//     fn mass(&self) -> f32 {
//         self.mass
//     }
//     fn pos(&self) -> [f32; 2] {
//         self.position
//     }
// }
