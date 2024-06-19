use crate::*;
use bevy::math::*;
use bevy::prelude::*;

// block order:
// I O T S Z J L
#[allow(unused)]
pub mod tetris {
    pub const I: usize = 0;
    pub const O: usize = 1;
    pub const T: usize = 2;
    pub const S: usize = 3;
    pub const Z: usize = 4;
    pub const J: usize = 5;
    pub const L: usize = 6;
}

pub const BLOCK_RECT_START: [Vec2; 7] = [
    vec2(0.0, 0.0),
    vec2(8.0, 0.0),
    vec2(8.0 * 2.0, 0.0),
    vec2(8.0 * 3.0, 0.0),
    vec2(8.0 * 4.0, 0.0),
    vec2(8.0 * 5.0, 0.0),
    vec2(8.0 * 6.0, 0.0),
];
pub const BLOCK_SIZE: Vec2 = vec2(8.0, 8.0);
pub const GRID_SIZE: Vec2 = vec2(
    field::GRID_WIDTH as f32 * BLOCK_SIZE.x,
    field::GRID_HEIGHT as f32 * BLOCK_SIZE.y,
);

pub const BLOCK_POSITIONS: [[Vec2; 4]; 7] = [
    [
        vec2(-1.5, 0.5),
        vec2(-0.5, 0.5),
        vec2(0.5, 0.5),
        vec2(1.5, 0.5),
    ],
    [
        vec2(-0.5, 0.5),
        vec2(0.5, 0.5),
        vec2(0.5, -0.5),
        vec2(-0.5, -0.5),
    ],
    [
        vec2(0.0, 1.0),
        vec2(0.0, 0.0),
        vec2(-1.0, 0.0),
        vec2(1.0, 0.0),
    ],
    [
        vec2(1.0, 1.0),
        vec2(0.0, 1.0),
        vec2(0.0, 0.0),
        vec2(-1.0, 0.0),
    ],
    [
        vec2(-1.0, 1.0),
        vec2(0.0, 1.0),
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
    ],
    [
        vec2(-1.0, 1.0),
        vec2(-1.0, 0.0),
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
    ],
    [
        vec2(-1.0, 0.0),
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
    ],
];

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct NextTetris;

#[derive(Component)]
pub struct ActiveTetris;

fn get_spawn_position(tetris_index: usize) -> Vec2 {
    (if matches!(tetris_index, tetris::I | tetris::O) {
        vec2(0.0, -1.0)
    } else {
        vec2(-0.5, -1.5)
    }) * BLOCK_SIZE
}
fn spawn_tetris(
    mut commands: Commands,
    tetris_index: usize,
    sprite_handle: Res<SpriteHandle>,
) -> Entity {
    let root = commands.spawn(TransformBundle::default()).id();

    let min = BLOCK_RECT_START[tetris_index];
    let max = min + BLOCK_SIZE;

    for i in 0..4 {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect::from_corners(min, max)),
                    ..Default::default()
                },
                transform: Transform::from_translation(
                    (BLOCK_POSITIONS[tetris_index][i] * BLOCK_SIZE).extend(0.0),
                ),
                texture: sprite_handle.0.clone(),
                ..Default::default()
            })
            .set_parent(root);
    }

    root
}
pub fn setup(
    mut commands: Commands,
    sprite_handle: Res<SpriteHandle>,
    manager: Res<TetrisManager>,
    field_q: Query<&Transform, With<field::Field>>,
    next_field_q: Query<&Transform, With<field::NextField>>,
) {
    let field_transform = field_q.single();

    let index = manager.current_tetris();
    let tetris = spawn_tetris(commands.reborrow(), index, Res::clone(&sprite_handle));
    commands.entity(tetris).insert((
        ActiveTetris {},
        Transform::from_translation(
            field_transform.translation + get_spawn_position(index).extend(0.0),
        ),
    ));

    let next_field_transform = next_field_q.single();

    let next_tetris = spawn_tetris(commands.reborrow(), manager.next_tetris(), sprite_handle);
    commands.entity(next_tetris).insert((
        NextTetris {},
        Transform::from_translation(next_field_transform.translation),
    ));
}
// check if point is colliding with the tetris blocks and walls
// WARNING: point must be relative to grid field's position!
pub fn is_colliding(
    point: Vec2,
    field_pos: Vec2,
    block_q: &Query<&Transform, With<Block>>,
) -> bool {
    println!("{}", point);

    let relative_point = point - field_pos;

    if relative_point.x.abs() > GRID_SIZE.x * 0.5 || relative_point.y < -GRID_SIZE.y {
        return true; // out of grid
    }

    let half_size = BLOCK_SIZE * 0.5;

    for transform in block_q.iter() {
        let block_point = transform.translation;
        let x_diff = (point.x - block_point.x).abs();
        let y_diff = (point.y - block_point.y).abs();

        if x_diff <= half_size.x && y_diff <= half_size.y {
            return true;
        }
    }

    false
}
pub fn is_tetris_colliding(
    transform: Transform,
    field_q: &Query<&Transform, With<field::Field>>,
    block_q: &Query<&Transform, With<Block>>,
    children_q: &Query<&Children, With<ActiveTetris>>,
    transform_q: &Query<&GlobalTransform>,
) -> bool {
    let children = children_q.single();
    let field_pos = field_q.single().translation;

    for &child in children {
        let child_transform = transform_q.get(child).unwrap();
        let point = transform.transform_point(child_transform.translation());
        if is_colliding(point.truncate(), field_pos.truncate(), &block_q) {
            return true;
        }
    }

    false
}
pub fn advance(
    mut commands: Commands,
    mut manager: ResMut<TetrisManager>,
    mut game_state: ResMut<GameState>,
    sprite_handle: Res<SpriteHandle>,
    active_tetris_q: Query<(Entity, &Children), With<ActiveTetris>>,
    next_tetris_q: Query<Entity, With<NextTetris>>,
    field_q: Query<&Transform, With<field::Field>>,
    next_field_q: Query<&Transform, With<field::NextField>>,
) {
    manager.advance();
    manager.fall_timer.reset();

    let field = field_q.single();

    let (old_tetris, old_tetris_children) = active_tetris_q.single();
    for &block in old_tetris_children {
        commands
            .entity(block)
            .remove_parent_in_place()
            .insert(Block {});
    }
    commands.entity(old_tetris).despawn();

    let new_tetris = next_tetris_q.single();
    commands.entity(new_tetris).remove::<NextTetris>().insert((
        Transform::from_translation(
            field.translation + get_spawn_position(manager.current_tetris()).extend(0.0),
        ),
        ActiveTetris {},
    ));

    let next_field = next_field_q.single();

    let new_next_tetris = spawn_tetris(commands.reborrow(), manager.next_tetris(), sprite_handle);
    commands.entity(new_next_tetris).insert((
        NextTetris {},
        Transform::from_translation(next_field.translation),
    ));

    *game_state = GameState::Play;
}
pub fn tetris_fall(
    time: Res<Time>,
    mut manager: ResMut<TetrisManager>,
    mut game_state: ResMut<GameState>,
    mut tetris_q: Query<
        &mut Transform,
        (With<ActiveTetris>, Without<field::Field>, Without<Block>),
    >,
    children_q: Query<&Children, With<ActiveTetris>>,
    field_q: Query<&Transform, With<field::Field>>,
    block_q: Query<&Transform, With<Block>>,
    transform_q: Query<&GlobalTransform>,
) {
    let delta = time.delta();

    manager.fall_timer.tick(delta);

    if manager.fall_timer.finished() {
        if is_tetris_colliding(
            Transform::from_translation(vec3(0.0, -BLOCK_SIZE.y, 0.0)),
            &field_q,
            &block_q,
            &children_q,
            &transform_q,
        ) {
            *game_state = GameState::BlockClear;
            return;
        }

        tetris_q.single_mut().translation.y -= BLOCK_SIZE.y;
    }
}
