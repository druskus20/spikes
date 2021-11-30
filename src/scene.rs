use bevy::prelude::*;

use crate::game;
use crate::main_menu;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(main_menu::MainMenuPlugin);
        app.add_plugin(game::GamePlugin);
    }
}
