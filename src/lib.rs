use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::window::WindowTheme;

const SPEED: f32 = 200.0;

#[derive(Component)]
struct Title;

#[derive(Component)]
pub struct NPC;

#[derive(Component)]
pub struct Player;

enum MoveState {
    HALT,
    RUN,
}

struct MoveRange {
    begin: usize,
    end: usize,
}

#[derive(Component)]
pub struct AtlasParas {
    state: MoveState,
    range: MoveRange,
    timer: Timer,
}

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
pub fn setup_systems(mut commands: Commands,
                     asset_server: Res<AssetServer>,
                     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Title,
        ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sensei.png")),
        Transform::from_xyz(100.0, 0.0, 1.0).with_scale(Vec3::splat(1.2)),
        //Health(100),
        NPC,
    ));
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None));
    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("gabe-idle-run.png"),
            TextureAtlas { layout, index: 0 },
        ),
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(1.44)),
        AtlasParas {state: MoveState::HALT,
                    range: MoveRange{begin: 1, end: 6},
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating)},
        Player,
    ));
}

pub fn exit_system(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}

pub fn move_sprite_system(
    time: Res<Time>,
    query: Single<(&mut Sprite, &mut Transform, &mut AtlasParas), With<Player>>,
    input: Res<ButtonInput<KeyCode>>
) {
    let (mut sprite, mut transform, mut atlas_paras) = query.into_inner();
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
    match &mut sprite.texture_atlas {
        Some(atlas) => if x == 0. && y == 0. {
                atlas.index = 0;
                atlas_paras.state = MoveState::HALT;
            } else {
                atlas_paras.state = MoveState::RUN;
                atlas_paras.timer.tick(time.delta());
                if atlas_paras.timer.just_finished() {
                    if atlas.index != atlas_paras.range.end {
                        atlas.index += 1;
                    } else { atlas.index = atlas_paras.range.begin; }
                }
            }
        None => (),
    };
    Vec3::new(x, y, 0.).try_normalize().map(|v| {
        transform.translation += v *delta;
    });
}