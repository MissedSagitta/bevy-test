use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup_systems)
        .add_systems(Update, update_systems)
        .run();
}

fn startup_systems() {
    println!("开始");
}

fn update_systems() {
    println!("刷新");
}
