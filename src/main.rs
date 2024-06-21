mod field;
mod tetris;

use std::mem;

use bevy::{core::*, prelude::*, render::camera::*, window::*};

const SPRITES_PATH: &str = "./sprites.png";
const VISIBLE_FRAME: u32 = 5;
const FALL_TIME: f32 = 0.5;
const FAST_FALL_TIME: f32 = 1.0 / 15.0;
const SLIDE_START_TIME: f32 = 0.2;
const SLIDE_TIME: f32 = 1.0 / 15.0;

fn game_state_setup(mut commands: Commands) {
    commands.insert_resource(GameState::Play);
    commands.insert_resource(TetrisManager::new());
}
fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -10.0,
            far: 10.0,
            scaling_mode: ScalingMode::FixedVertical(field::FIELD_RECT.height()),
            ..Default::default()
        },
        ..Default::default()
    });
}
fn asset_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_handle = asset_server.load::<Image>(SPRITES_PATH);
    commands.insert_resource(SpriteHandle(sprite_handle));
}
fn make_visible(frame: Res<FrameCount>, mut window_q: Query<&mut Window>) {
    if frame.0 == VISIBLE_FRAME {
        for mut window in window_q.iter_mut() {
            window.visible = true;
        }
    }
}

#[derive(Resource)]
pub struct SpriteHandle(pub Handle<Image>);
#[derive(Resource)]
pub struct TetrisManager {
    pub order: [usize; 7],
    pub next_order: [usize; 7],
    pub order_index: usize,
    pub fall_timer: Timer,
    pub fast_fall_timer: Timer,
    pub slide_start_timer: Timer,
    pub slide_timer: Timer,
    pub slide_dir: f32,
}
impl TetrisManager {
    fn new() -> Self {
        let mut order = [0; 7];
        let mut next_order = [0; 7];

        // initialized here
        for i in 0..7 {
            order[i] = i;
            next_order[i] = i;
        }
        fastrand::shuffle(&mut order);
        fastrand::shuffle(&mut next_order);

        Self {
            order,
            next_order,
            order_index: 0,
            fall_timer: Timer::from_seconds(FALL_TIME, TimerMode::Repeating),
            fast_fall_timer: Timer::from_seconds(FAST_FALL_TIME, TimerMode::Repeating),
            slide_start_timer: Timer::from_seconds(SLIDE_START_TIME, TimerMode::Once),
            slide_timer: Timer::from_seconds(SLIDE_TIME, TimerMode::Repeating),
            slide_dir: 0.0,
        }
    }
    pub fn current_tetris(&self) -> usize {
        self.order[self.order_index]
    }
    pub fn next_tetris(&self) -> usize {
        if let Some(order) = self.order.get(self.order_index + 1) {
            return *order;
        }
        self.next_order[(self.order_index + 1) % self.order.len()]
    }
    pub fn advance(&mut self) {
        self.order_index += 1;
        if self.order_index >= self.order.len() {
            mem::swap(&mut self.order, &mut self.next_order);
            fastrand::shuffle(&mut self.next_order);
            self.order_index %= self.order.len();
        }
    }
}
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Play,
    BlockClear,
    GameOver,
}

fn is_state_play(game_state: Res<GameState>) -> bool {
    *game_state == GameState::Play
}
fn is_state_block_clear(game_state: Res<GameState>) -> bool {
    *game_state == GameState::BlockClear
}
fn is_state_game_over(game_state: Res<GameState>) -> bool {
    *game_state == GameState::GameOver
}

fn main() {
    println!("Current direction {:?}", std::env::current_dir().unwrap());

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(800.0, 600.0),
                        resizable: false,
                        title: "Tetry".to_string(),
                        visible: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(
            Startup,
            (
                asset_setup,
                camera_setup,
                (field::setup.after(asset_setup), game_state_setup).before(tetris::setup),
                tetris::setup,
            ),
        )
        .add_systems(
            Update,
            (
                make_visible,
                tetris::place.run_if(is_state_block_clear).before(tetris::advance),
                tetris::advance.run_if(is_state_block_clear),
                (
                    tetris::fall.run_if(is_state_play),
                    tetris::slide.run_if(is_state_play),
                    tetris::rotate.run_if(is_state_play),
                )
                    .after(tetris::advance),
            ),
        )
        .run();
}
