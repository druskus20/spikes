use bevy::prelude::*;

pub struct GameDataPlugin;
impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::MainMenu)
            .insert_resource(Score { score: -1 });
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    MainMenu,
    Game,
    GameOver,
}

struct Score {
    score: i32,
}
