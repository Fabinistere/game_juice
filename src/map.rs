use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

use crate::{
    collisions::{TesselatedCollider, TesselatedColliderConfig},
    GameState,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), map_setup);
    }
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let colliders: Vec<(Handle<Image>, &str)> = vec![
        (
            asset_server.load("textures/map/colliders/big roof part.png"),
            "big roof part",
        ),
        (
            asset_server.load("textures/map/colliders/box left.png"),
            "box left",
        ),
        (
            asset_server.load("textures/map/colliders/box middle roof.png"),
            "box middle roof",
        ),
        (
            asset_server.load("textures/map/colliders/box roof.png"),
            "box roof",
        ),
        (
            asset_server.load("textures/map/colliders/first step right.png"),
            "first step right",
        ),
        (
            asset_server.load("textures/map/colliders/left first step roof.png"),
            "left first step roof",
        ),
        (
            asset_server.load("textures/map/colliders/left floor.png"),
            "left floor",
        ),
        (
            asset_server.load("textures/map/colliders/left lower wall.png"),
            "left lower wall",
        ),
        (
            asset_server.load("textures/map/colliders/left roof roof.png"),
            "left roof roof",
        ),
        (
            asset_server.load("textures/map/colliders/left roof.png"),
            "left roof",
        ),
        (
            asset_server.load("textures/map/colliders/left second step roof.png"),
            "left second step roof",
        ),
        (
            asset_server.load("textures/map/colliders/left third step roof.png"),
            "left third step roof",
        ),
        (
            asset_server.load("textures/map/colliders/lower floor.png"),
            "lower floor",
        ),
        (
            asset_server.load("textures/map/colliders/lower left roof.png"),
            "lower left roof",
        ),
        (
            asset_server.load("textures/map/colliders/middle roof intersection.png"),
            "middle roof intersection",
        ),
        (
            asset_server.load("textures/map/colliders/middle roof roof.png"),
            "middle roof roof",
        ),
        (
            asset_server.load("textures/map/colliders/right first step roof.png"),
            "right first step roof",
        ),
        (
            asset_server.load("textures/map/colliders/right floor.png"),
            "right floor",
        ),
        (
            asset_server.load("textures/map/colliders/right roof part.png"),
            "right roof part",
        ),
        (
            asset_server.load("textures/map/colliders/right roof roof.png"),
            "right roof roof",
        ),
        (
            asset_server.load("textures/map/colliders/right second step roof.png"),
            "right second step roof",
        ),
        (
            asset_server.load("textures/map/colliders/second step right.png"),
            "second step right",
        ),
        (
            asset_server.load("textures/map/colliders/third step right.png"),
            "third step right",
        ),
    ];
    let walls = asset_server.load("textures/map/Mosaic_demo__Walls.png");
    let background = asset_server.load("textures/map/Mosaic_demo__Background.png");

    commands
        .spawn((
            SpriteBundle {
                texture: background,
                ..default()
            },
            Name::new("Map - Background"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    SpriteBundle {
                        texture: walls,
                        ..default()
                    },
                    Name::new("Map Walls"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Transform::IDENTITY,
                            Name::new("Colliders"),
                            RigidBody::Fixed,
                        ))
                        .with_children(|parent| {
                            for (image, name) in colliders {
                                parent.spawn((
                                    TesselatedCollider {
                                        texture: image.clone(),
                                        tesselator_config: TesselatedColliderConfig {
                                            vertice_separation: 0.,
                                            extrusion: 0.1,
                                            vertice_radius: 0.4,
                                        },
                                    },
                                    Transform::IDENTITY,
                                    Name::new(format!("{name}")),
                                ));
                            }
                        });
                });
        });
}
