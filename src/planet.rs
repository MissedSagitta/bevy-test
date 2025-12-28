use bevy::prelude::*;

pub mod planet_plugin;

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Planet {
    pub name: String,
    pub size: u32,
    init_pos: (f32, f32),
    moving: (KeyCode, KeyCode, KeyCode, KeyCode),
}