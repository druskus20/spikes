use bevy::prelude::*;

use crate::{end::EndPlugin, game::GamePlugin, title::TitlePlugin};

pub struct ScreensPlugin;
impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(Screen::Title)
            .add_plugin(TitlePlugin)
            .add_plugin(GamePlugin)
            .add_plugin(EndPlugin);
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum Screen {
    Title,
    Game,
    End,
}
