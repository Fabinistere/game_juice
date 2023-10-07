#![allow(clippy::type_complexity)]

use bevy::{asset::ChangeWatcher, ecs::schedule::ScheduleBuildSettings, prelude::*};
use bevy_rapier2d::prelude::*;
use debug::DebugPlugin;
use std::time::Duration;

use crate::{
    characters::player::PlayerPlugin, collisions::CollisionsPlugin, controls::Key, map::MapPlugin,
};

mod animations;
mod characters;
mod collisions;
mod constants;
mod controls;
mod debug;
mod map;
mod movement;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Reflect, States)]
pub enum GameState {
    #[default]
    Playing,
}

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(controls::KeyBindings {
            up: [Key(KeyCode::W), Key(KeyCode::Z), Key(KeyCode::Up)],
            down: [Key(KeyCode::S), Key(KeyCode::Down)],
            right: [Key(KeyCode::D), Key(KeyCode::Right)],
            left: [Key(KeyCode::A), Key(KeyCode::Q), Key(KeyCode::Left)],
            interact: [Key(KeyCode::E), Key(KeyCode::R)],
        })
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game Juice".to_string(),
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        // vsync: true,
                        ..Window::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
            // bevy_tweening::TweeningPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.),
            // ----- Our plugins -----
            CollisionsPlugin,
            DebugPlugin,
            animations::AnimationPlugin,
            MapPlugin,
            PlayerPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, game_setup);

    app.edit_schedule(Main, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: bevy::ecs::schedule::LogLevel::Warn,
            ..default()
        });
    });

    app.run();
}

#[derive(Component)]
struct PlayerCamera;

fn game_setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vect::ZERO;

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.1;
    commands.spawn((camera, PlayerCamera));
}

/* -------------------------------------------------------------------------- */
/*                                   Run If                                   */
/* -------------------------------------------------------------------------- */

pub fn playing(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Playing
}
