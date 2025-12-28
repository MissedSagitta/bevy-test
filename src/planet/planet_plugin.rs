use bevy::ecs::event::Trigger;
use bevy::prelude::*;

use super::*;

const SPEED: f32 = 800.;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer()
            .add_systems(Update, moving_planet);
    }
}

fn moving_planet(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_query: Query<(&mut Transform, &Sprite, &Planet)>,
) {
    let delta = time.delta_secs()*SPEED;

    sprite_query.iter_mut().for_each( |(mut transform, _, planet)| {
        let mut x = 0f32;
        let mut y = 0f32;
        if keyboard_input.pressed(planet.moving.0) {
            x -= 1.;
        }
        if keyboard_input.pressed(planet.moving.1) {
            y += 1.;
        }
        if keyboard_input.pressed(planet.moving.2) {
            x += 1.;
        }
        if keyboard_input.pressed(planet.moving.3) {
            y -= 1.;
        }
        Vec3::new(x, y, 0.).try_normalize().map(|v| {
            transform.translation += v *delta;
        });
    });
}

fn ob_planet(
    changed_query: Query<&Planet, Added<Planet>>,
) {

}