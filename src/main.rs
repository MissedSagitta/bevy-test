use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

mod planet;
use bevy_test::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(windows_paras()))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup_systems)
        .add_systems(Update, (
            move_sprite_system,
            exit_system.run_if(input_just_pressed(KeyCode::Escape))
        ))
        .run();
}