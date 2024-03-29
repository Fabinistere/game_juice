use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animations::{
        sprite_sheet_animation::{AnimationIndices, CharacterState},
        CharacterSpriteSheet,
    },
    constants::{
        character::player::{
            CAMERA_INTERPOLATION, PLAYER_IDLE_FRAMES, PLAYER_RUN_FRAMES, PLAYER_SCALE,
        },
        TILE_SIZE,
    },
    controls::KeyBindings,
    movement::{MovementBundle, Speed},
    playing, GameState, PlayerCamera,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (player_movement.run_if(playing), camera_follow));
    }
}

#[derive(Component)]
pub struct Player;

fn player_movement(
    key_bindings: Res<KeyBindings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            Entity,
            &Speed,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &mut CharacterState,
        ),
        With<Player>,
    >,
) {
    if let Ok((_player, speed, mut rb_vel, mut texture_atlas_sprite, mut player_state)) =
        player_query.get_single_mut()
    {
        let up = keyboard_input.any_pressed(key_bindings.up());
        let down = keyboard_input.any_pressed(key_bindings.down());
        let left = keyboard_input.any_pressed(key_bindings.left());
        let right = keyboard_input.any_pressed(key_bindings.right());

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut vel_x = x_axis as f32 * **speed;
        let mut vel_y = y_axis as f32 * **speed;

        if x_axis != 0 && y_axis != 0 {
            vel_x *= (std::f32::consts::PI / 4.).cos();
            vel_y *= (std::f32::consts::PI / 4.).cos();
        }

        // rb_vel.linvel.x = x_axis as f32 * **speed * 200. * time.delta_seconds();
        rb_vel.linvel.x = vel_x;
        rb_vel.linvel.y = vel_y;

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        // if there is any movement
        if (left || right || up || down) && *player_state != CharacterState::Run {
            *player_state = CharacterState::Run;
        } else if !(left || right || up || down)
            && *player_state == CharacterState::Run
            && *player_state != CharacterState::Idle
        {
            // IDEA: Polish #visual - When we reach max speed (one full run loop), whenever you stop there is a smoke anim (sudden braking)
            *player_state = CharacterState::Idle;
        }

        /* -------------------------------------------------------------------------- */
        /*                                  Direction                                 */
        /* -------------------------------------------------------------------------- */

        if !(left && right) {
            if right {
                texture_atlas_sprite.flip_x = false;
            } else if left {
                texture_atlas_sprite.flip_x = true;
            }
        }
    }
}

fn camera_follow(
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<PlayerCamera>>,
    )>,
) {
    if let Ok(t) = query.p0().get_single() {
        let player_transform = *t;

        if let Ok(mut camera_transform) = query.p1().get_single_mut() {
            camera_transform.translation = camera_transform.translation.lerp(
                Vec3::new(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    camera_transform.translation.z,
                ),
                CAMERA_INTERPOLATION,
            );
        }
    }
}

fn spawn_player(mut commands: Commands, characters_spritesheet: Res<CharacterSpriteSheet>) {
    /* -------------------------------------------------------------------------- */
    /*                              Animation Indices                             */
    /* -------------------------------------------------------------------------- */

    let mut animation_indices = AnimationIndices(HashMap::new());
    animation_indices.insert(CharacterState::Idle, PLAYER_IDLE_FRAMES);
    animation_indices.insert(CharacterState::Run, PLAYER_RUN_FRAMES);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: characters_spritesheet.texture_atlas.clone(),
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE)),
                ..default()
            },
            Name::new("Player"),
            Player,
            // -- Animation --
            MovementBundle {
                animation_indices,
                speed: Speed(100. * TILE_SIZE),
                ..default()
            },
            // -- Hitbox --
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|_parent| {
            // collider
        });
}
