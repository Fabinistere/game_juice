pub const RESOLUTION: f32 = 9. / 16.; // 16. / 9.;
pub const TILE_SIZE: f32 = 1.;

pub const FRAME_TIME: f32 = 0.1;

pub mod character {
    // pub const CHAR_SCALE: f32 = 0.6 * super::TILE_SIZE;
    pub const CHAR_SCALE: f32 = 1. * super::TILE_SIZE;

    pub const CHAR_HITBOX_HEIGHT: f32 = 1.5 * CHAR_SCALE;
    pub const CHAR_HITBOX_WIDTH: f32 = 5. * CHAR_SCALE;
    pub const CHAR_HITBOX_Y_OFFSET: f32 = -6.25;
    pub const CHAR_SENSOR_Y_OFFSET: f32 = -1.25;

    pub const COLUMN_FRAME_IDLE_START: usize = 1;
    pub const COLUMN_FRAME_IDLE_END: usize = 6;
    pub const COLUMN_FRAME_RUN_START: usize = 7;
    pub const COLUMN_FRAME_RUN_END: usize = 14;
    pub const COLUMN_FRAME_SHOOT_START: usize = 15;
    pub const COLUMN_FRAME_SHOOT_END: usize = 20;
    pub const COLUMN_FRAME_HIT_START: usize = 27;
    pub const COLUMN_FRAME_HIT_END: usize = 28;
    pub const COLUMN_FRAME_MELEE_START: usize = 29;
    pub const COLUMN_FRAME_MELEE_END: usize = 34;
    pub const COLUMN_FRAME_DEATH_START: usize = 35;
    pub const COLUMN_FRAME_DEATH_END: usize = 46;

    pub const SPRITESHEET_LINE_NUMBER: usize = 1;
    pub const SPRITESHEET_COLUMN_NUMBER: usize = 47;

    pub mod player {
        use crate::animations::sprite_sheet_animation::CharacterState;

        use super::{
            COLUMN_FRAME_IDLE_END, COLUMN_FRAME_IDLE_START, COLUMN_FRAME_RUN_START, COLUMN_FRAME_RUN_END,
            SPRITESHEET_COLUMN_NUMBER,
        };

        pub const PLAYER_WIDTH: f32 = 12.;
        pub const PLAYER_HEIGHT: f32 = 15.;
        pub const PLAYER_SCALE: f32 = super::CHAR_SCALE;
        pub const PLAYER_SPAWN: (f32, f32, f32) = (-24., -150., 0.);

        pub const CAMERA_INTERPOLATION: f32 = 0.1;

        /* -------------------------------------------------------------------------- */
        /*                                  Animation                                 */
        /* -------------------------------------------------------------------------- */

        pub const PLAYER_LINE: usize = 0;
        pub const PLAYER_LINE_START: usize = PLAYER_LINE * SPRITESHEET_COLUMN_NUMBER;
        // (start_frame, end_frame, next_state)
        pub const PLAYER_RUN_FRAMES: (usize, usize, CharacterState) = (
            PLAYER_LINE_START + COLUMN_FRAME_RUN_START,
            PLAYER_LINE_START + COLUMN_FRAME_RUN_END,
            // CharacterState::Idle,
            CharacterState::Run,
        );
        pub const PLAYER_IDLE_FRAMES: (usize, usize, CharacterState) = (
            PLAYER_LINE_START + COLUMN_FRAME_IDLE_START,
            PLAYER_LINE_START + COLUMN_FRAME_IDLE_END,
            CharacterState::Idle,
        );
    }

    pub mod npcs {

        pub const NPC_SCALE: f32 = super::CHAR_SCALE;

        pub mod movement {
            use crate::constants::TILE_SIZE;

            pub const NPC_SPEED: f32 = 50. * TILE_SIZE; // -> Speed::default()
        }
    }
}

pub mod locations {}
