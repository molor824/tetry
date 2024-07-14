use bevy::math::*;
use bevy::prelude::*;
use bevy::sprite::*;

use crate::tetris;
use crate::SpriteHandle;

pub const GRID_WIDTH: u32 = 10;
pub const GRID_HEIGHT: u32 = 20;
pub const FIELD_RECT: Rect = Rect {
    min: vec2(64.0, 0.0),
    max: vec2(176.0, 176.0),
};
pub const NEXT_FIELD_RECT: Rect = Rect {
    min: vec2(16.0, 16.0),
    max: vec2(64.0, 64.0),
};
pub const HOLD_FIELD_RECT: Rect = NEXT_FIELD_RECT;

#[derive(Component)]
pub struct Field;

#[derive(Component)]
pub struct NextField;

#[derive(Component)]
pub struct HoldField;

pub fn setup(mut commands: Commands, sprite_handle: Res<SpriteHandle>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite_handle.0.clone(),
            sprite: Sprite {
                rect: Some(FIELD_RECT),
                anchor: Anchor::TopCenter,
                ..Default::default()
            },
            transform: Transform::from_translation(vec3(0.0, tetris::GRID_SIZE.y * 0.5, 0.0)),
            ..Default::default()
        },
        Field,
    ));
    commands.spawn((
        SpriteBundle {
            texture: sprite_handle.0.clone(),
            sprite: Sprite {
                rect: Some(NEXT_FIELD_RECT),
                ..Default::default()
            },
            transform: Transform {
                translation: (FIELD_RECT.size() * 0.5 + NEXT_FIELD_RECT.size() * vec2(0.5, -0.5))
                    .extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        NextField,
    ));
    commands.spawn((
        SpriteBundle {
            texture: sprite_handle.0.clone(),
            sprite: Sprite {
                rect: Some(HOLD_FIELD_RECT),
                ..Default::default()
            },
            transform: Transform {
                translation: (FIELD_RECT.size() * vec2(-0.5, 0.5)
                    + HOLD_FIELD_RECT.size() * vec2(-0.5, -0.5))
                .extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        HoldField,
    ));
}
