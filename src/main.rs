use bevy::{app::Events, prelude::*, render::pass::ClearColor, window::WindowResized};

#[allow(dead_code)]
mod game;
mod scene;
mod title;

pub const TITLE: &str = "Spikes";
pub const WIDTH: f32 = 500.0;
pub const HEIGHT: f32 = 500.0;

fn main() {
    App::build()
        .set_default_properties()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_cameras.system())
        .add_plugin(game::GamePlugin)
        .add_plugin(scene::ScenePlugin)
        .run();
}

trait AppBuilderExt {
    fn set_default_properties(&mut self) -> &mut Self;
}

impl AppBuilderExt for AppBuilder {
    fn set_default_properties(&mut self) -> &mut Self {
        self.insert_resource(Msaa { samples: 4 })
            .insert_resource(WindowDescriptor {
                title: TITLE.to_string(),
                width: WIDTH,
                height: HEIGHT,
                ..Default::default()
            })
            .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    }
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
