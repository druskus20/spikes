use bevy::{prelude::*, render::pass::ClearColor};

mod scenes;
use scenes::*;

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(load_scene_system.system())
        .add_startup_system(save_scene_system.exclusive_system())
        .add_startup_system(game::setup.system())
        .add_system(title::title_screen.system())
        .add_system(game::paddle_movement_system.system())
        .add_system(game::ball_collision_system.system())
        .add_system(game::ball_movement_system.system())
        .add_system(game::scoreboard_system.system())
        .run();
}

#[derive(Reflect, Clone)]
pub struct Scoreboard {
    score: usize,
}
