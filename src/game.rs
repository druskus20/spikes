use bevy::prelude::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        //app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup.system()));
        app.add_state(GameState::MainMenu)
            .insert_resource(Score { score: -1 });
    }
}

fn setup(mut commands: Commands) {}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    MainMenu,
    Game,
    GameOver,
}

struct Score {
    score: i32,
}
