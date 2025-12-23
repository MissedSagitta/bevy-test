use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::window::WindowTheme;

const SPEED: f32 = 200.0;

#[derive(Component)]
struct Title;

#[derive(Component)]
pub struct NPC;

//#[derive(Component)]
//struct Health(i32);

pub fn windows_paras() -> WindowPlugin {
    WindowPlugin {primary_window: Some(Window {
            title: "bevy test".to_string(),
            resolution: (1280, 810).into(),
            window_theme: Some(WindowTheme::Light),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }
}
pub fn setup_systems(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Title,
        ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sensei.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
        //Health(100),
        NPC,
    ));
}

pub fn exit_system(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

pub fn move_sprite_system(
    time: Res<Time>,
    mut query: Single<&mut Transform, With<NPC>>,
    input: Res<ButtonInput<KeyCode>>)
{
    let delta = time.delta_secs()*SPEED;
    let mut x = 0f32;
    let mut y = 0f32;

    if input.pressed(KeyCode::KeyA) {
        x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        x += 1.;
    }
    if input.pressed(KeyCode::KeyW) {
        y += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        y -= 1.;
    }
    if x != 0. || y != 0. {
        query.translation += Vec3::new(x, y, 0.).normalize() * delta;
    }

}