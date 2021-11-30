use bevy::prelude::*;

mod end;
mod game;
mod main_menu;

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(Screen::MainMenu)
            .add_plugin(main_menu::MainMenuPlugin)
            .add_plugin(game::GamePlugin)
            .add_plugin(end::EndPlugin);
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum Screen {
    MainMenu,
    Game,
    EndScreen,
}
