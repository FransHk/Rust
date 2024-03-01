use bevy::prelude::*;
mod celestial_bodies;
mod utils;
use celestial_bodies::body_config::*;
use celestial_bodies::planet::Planet;
use utils::colour::Colour;
use utils::physics::grav_force;

const WHITE: Colour = [1.0; 4];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

fn gen_sprite(
    planet: &Planet,
    asset_server: &Res<AssetServer>,
    sprite_path: String,
) -> SpriteBundle {
    let sprite = SpriteBundle {
        texture: asset_server.load(sprite_path),
        transform: Transform {
            scale: Vec3::new(planet.size[0], planet.size[1], 0.),
            translation: Vec3::new(planet.position[0], planet.position[1], 0.),
            rotation: Quat::from_rotation_z(60.0),
            ..default()
        },
        ..default()
    };

    sprite
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planet_const = PlanetConfig::new(0.0, 300.0, 50.0, 10.0, 5.0, 0.01);
    let sun_const = PlanetConfig::new(-150., 150.0, 0., 7000.0, 5.0, 0.0001);
    let planet_amt = 5;
    // let sprite_sun = SpriteBundle {
    //     texture: asset_server.load("cat-modified.png"),
    //     transform: Transform {
    //         scale: Vec3::new(0.2, 0.2, 0.2),
    //         translation: Vec3::new(0.0, 0.0, 0.0),
    //         rotation: Quat::IDENTITY,
    //         ..default()
    //     },
    //     ..default()
    // };
    commands.spawn(Camera2dBundle::default());
    for i in 0..planet_amt {
        let planet = Planet::new(&planet_const, i);
        let sprite = gen_sprite(&planet, &asset_server, "cat-modified.png".to_string());
        commands.spawn((planet.clone(), sprite.clone()));
    }

    let sun = Planet::new(&sun_const, planet_amt);
    let sprite = gen_sprite(&sun, &asset_server, "sun.png".to_string());
    commands.spawn((sun.clone(), sprite.clone()));
}

fn sprite_movement(time: Res<Time>, mut planets: Query<(Entity, &mut Planet, &mut Transform)>) {
    let mut entity_ids: Vec<Entity> = Vec::new();
    for (e, mut planet, mut transform) in &mut planets {
        planet.update(time.delta_seconds());

        transform.translation.x = planet.position[0];
        transform.translation.y = planet.position[1];
        planet.check_dist_from_centre([0.0, 0.0]);
        entity_ids.push(e);
    }
    let mut bottom: usize = 1;
    for i in 0..entity_ids.len() {
        for j in bottom..entity_ids.len() {
            if i != j {
                let entity_1 = planets.get(entity_ids[i]).expect("");
                let entity_2 = planets.get(entity_ids[j]).expect("");
                let (force, force_inv) = grav_force(&entity_1.1, &entity_2.1, 1.);
                planets
                    .get_mut(entity_ids[i])
                    .expect("")
                    .1
                    .add_force(force_inv);

                planets.get_mut(entity_ids[j]).expect("").1.add_force(force);
            }
        }
        bottom += 1;
    }
}
