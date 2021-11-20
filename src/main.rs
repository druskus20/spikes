use bevy::{app::Events, prelude::*, render::pass::ClearColor, window::WindowResized};

mod scenes;

pub const WIN_WIDTH: f32 = 1000.0;
pub const WIN_HEIGHT: f32 = 1000.0;

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        // TODO: Move this
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Spike".to_string(),
            width: WIN_WIDTH,
            height: WIN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        // ---
        .add_plugins(DefaultPlugins)
        .add_plugin(scenes::ScenesPlugin)
        .run();
}
