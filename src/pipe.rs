use bevy::prelude::*;
use rand::Rng;

use crate::{state::AppState, Collider, Velocity};

const PIPE_SIZE: Vec2 = Vec2::new(52.0, 320.0);

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeTimer {
            spawn: Timer::from_seconds(1.7, TimerMode::Repeating),
        })
        .insert_resource(PipeAssets::default())
        .insert_resource(RandomPipe::default())
        .add_systems(Startup, load_pipe_texture)
        .add_systems(
            Update,
            (spawn_pipes, move_pipes, despawn_pipes)
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Default)]
pub struct PipeAssets {
    pub green: Option<Handle<Image>>,
    pub red: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct RandomPipe {
    pub texture: Option<Handle<Image>>,
}

#[derive(Resource)]
struct PipeTimer {
    spawn: Timer,
}

#[derive(Component)]
pub struct TopPipe;

#[derive(Component)]
pub struct BottomPipe;

fn load_pipe_texture(asset_server: Res<AssetServer>, mut pipes: ResMut<PipeAssets>) {
    pipes.green = Some(asset_server.load("sprites/pipes/pipe-green.png"));
    pipes.red = Some(asset_server.load("sprites/pipes/pipe-red.png"));
}

fn spawn_pipes(
    mut commands: Commands,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    pipe_assets: Res<PipeAssets>,
    random_pipe: Res<RandomPipe>,
    mut timer: ResMut<PipeTimer>,
    time: Res<Time>,
) {
    let window = window_query.single();
    let texture = if random_pipe.texture.as_ref().is_some() {
        random_pipe
            .texture
            .as_ref()
            .expect("Embedded in the binary")
    } else {
        pipe_assets.green.as_ref().expect("Embedded in the binary")
    };

    if timer.spawn.tick(time.delta()).just_finished() {
        let gap = window.height() * 0.1;
        let positions = [0.0, gap, -gap, gap * 2.0, gap * 3.0];
        let pipe_position = (window.height() / 2.0) - gap;
        let rng = rand::thread_rng().gen_range(0..positions.len());
        let offset = positions[rng];

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 2.0 + PIPE_SIZE.x,
                        (pipe_position) + offset,
                        1.0,
                    ),
                    rotation: Quat::from_rotation_x(180.0_f32.to_radians()),
                    ..default()
                },
                ..default()
            },
            Velocity {
                value: Vec2::new(100.0, 0.0),
            },
            Collider { size: PIPE_SIZE },
            TopPipe,
        ));

        commands.spawn((
            SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        window.width() / 2.0 + PIPE_SIZE.x,
                        (pipe_position * -1.0) + offset,
                        1.0,
                    ),
                    ..default()
                },

                ..default()
            },
            Velocity {
                value: Vec2::new(100.0, 0.0),
            },
            Collider { size: PIPE_SIZE },
            BottomPipe,
        ));
    }
}

fn move_pipes(
    mut query: Query<(&mut Transform, &Velocity), Or<(With<BottomPipe>, With<TopPipe>)>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x -= velocity.value.x * time.delta_seconds();
    }
}

fn despawn_pipes(
    mut commands: Commands,
    mut pipe_query: Query<(&Transform, Entity), Or<(With<BottomPipe>, With<TopPipe>)>>,
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
) {
    if let Ok(window) = window.get_single() {
        for (transform, entity) in pipe_query.iter_mut() {
            if transform.translation.x < -((window.width() / 2.0) + PIPE_SIZE.x) {
                commands.entity(entity).despawn();
            }
        }
    }
}
