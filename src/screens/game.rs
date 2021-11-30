use bevy::prelude::*;

use crate::gamedata::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup.system()))
            //.add_system_set(SystemSet::on_update(GameState::Game).with_system(system.system()))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn.system()));
    }
}

fn setup() {}
fn despawn() {}
