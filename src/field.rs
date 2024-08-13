use bevy::ecs::system::SystemChangeTick;
use bevy::math::*;
use bevy::prelude::*;
use bevy::sprite::*;

use std::fmt::Write;
use std::fs;

use crate::tetris;
use crate::*;

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

#[derive(Component, Clone, Copy)]
pub struct Score {
    pub best_score: u64,
    pub current_score: u64,
}
impl Score {
    pub fn new(best_score: u64) -> Self {
        Self {
            best_score,
            current_score: 0,
        }
    }
}

pub fn setup(
    mut commands: Commands,
    sprite_handle: Res<SpriteHandle>,
    font_handle: Res<FontHandle>,
) {
    let text_style = TextStyle {
        font: font_handle.0.clone(),
        font_size: 200.0,
        color: Color::WHITE,
    };
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
    commands
        .spawn((
            SpriteBundle {
                texture: sprite_handle.0.clone(),
                sprite: Sprite {
                    rect: Some(NEXT_FIELD_RECT),
                    ..Default::default()
                },
                transform: Transform {
                    translation: (FIELD_RECT.size() * 0.5
                        + NEXT_FIELD_RECT.size() * vec2(0.5, -0.5)
                        + vec2(0.0, -16.0))
                    .extend(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            NextField,
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new("Next", text_style.clone())],
                    ..Default::default()
                },
                text_anchor: Anchor::BottomCenter,
                transform: Transform {
                    translation: (NEXT_FIELD_RECT.size() * vec2(0.0, 0.5)).extend(0.0),
                    scale: Vec3::splat(12.0 / text_style.font_size),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    commands
        .spawn((
            SpriteBundle {
                texture: sprite_handle.0.clone(),
                sprite: Sprite {
                    rect: Some(HOLD_FIELD_RECT),
                    ..Default::default()
                },
                transform: Transform {
                    translation: (FIELD_RECT.size() * vec2(-0.5, 0.5)
                        + HOLD_FIELD_RECT.size() * vec2(-0.5, -0.5)
                        + vec2(0.0, -16.0))
                    .extend(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            HoldField,
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new("Hold", text_style.clone())],
                    ..Default::default()
                },
                text_anchor: Anchor::BottomCenter,
                transform: Transform {
                    translation: (HOLD_FIELD_RECT.size() * vec2(0.0, 0.5)).extend(0.0),
                    scale: Vec3::splat(12.0 / text_style.font_size),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection::new("Score\n0\n\nBest score\n0", text_style.clone()),
                ],
                justify: JustifyText::Left,
                ..Default::default()
            },
            text_anchor: Anchor::CenterLeft,
            transform: Transform {
                translation: (FIELD_RECT.size() * vec2(0.5, 0.0)).extend(0.0),
                scale: Vec3::splat(8.0 / text_style.font_size),
                ..Default::default()
            },
            ..Default::default()
        },
        Score::new(0),
    ));
}
pub fn load_score(mut score_q: Query<&mut Score>) {
    let mut score = score_q.single_mut();
    score.best_score = fs::read_to_string("./score").map(|s| s.parse::<u64>().unwrap()).unwrap_or(0);
}
pub fn update_score(mut score_q: Query<(&Score, &mut Text), Changed<Score>>) {
    for (score, mut text) in score_q.iter_mut() {
        info!("Updating score");

        text.sections[0].value.clear();
        write!(text.sections[0].value, "Score\n{}\n\nBest Score\n{}", score.current_score, score.best_score).unwrap();
    }
}
