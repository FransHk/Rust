use super::array_logic::{dot_product, get_length, normalise_vector, scalar_mult, subtract_arrays};
use crate::celestial_bodies::body_config::CelestialBody;

/// Calculate the gravitational force between two bodies,
/// takes two structs that implement the CelestialBody trait
/// this means that the body:
/// - Has mass
/// - Has pos
///
pub fn grav_force<C: CelestialBody>(mass1: &C, mass2: &C, g: f64) -> ([f64; 2], bool) {
    let dist = subtract_arrays(mass1.pos(), mass2.pos());
    let dist_length = get_length(dist);
    let sqr_dist = dot_product(dist, dist);
    let force_dir = normalise_vector(dist);
    let force = scalar_mult(force_dir, g * -1.0 * mass2.mass());
    let force = scalar_mult(force, 1.0 / sqr_dist);
    let colliding = dist_length <= 0.3;
    (force, colliding)
}
