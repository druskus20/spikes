use bevy::prelude::*;

mod game;
mod main_menu;

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(main_menu::MainMenuPlugin);
        app.add_plugin(game::GamePlugin);
    }
}
