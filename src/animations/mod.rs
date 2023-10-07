use bevy::prelude::*;

pub mod sprite_sheet_animation;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterSpriteSheet>().add_systems(
            PostUpdate,
            (
                sprite_sheet_animation::animate_sprite_sheet,
                sprite_sheet_animation::jump_frame_character_state,
                sprite_sheet_animation::tempo_animation_timer,
                sprite_sheet_animation::animate_character,
            ),
        );
    }
}

#[derive(Deref, Clone, Resource)]
pub struct CharacterSpriteSheet {
    pub texture_atlas: Handle<TextureAtlas>,
}

impl FromWorld for CharacterSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_handle = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load("textures/characters/Shotgunner_spritesheet.png");
        let atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(146., 36.), 47, 1, None, None);

        let atlas_handle = world
            .get_resource_mut::<Assets<TextureAtlas>>()
            .unwrap()
            .add(atlas);

        CharacterSpriteSheet {
            texture_atlas: atlas_handle,
        }
    }
}
