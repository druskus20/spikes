use core::time;

use bevy::prelude::*;

use super::Screen;
mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(Screen::Game).with_system(input.system()))
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()))
            .add_plugin(player::PlayerPlugin);
    }
}

fn setup() {
    dbg!("Game Setup");
}

fn despawn() {}

fn input(mut input_state: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<Screen>>) {
    if input_state.just_pressed(KeyCode::Space) {
        game_state.set(Screen::EndScreen).unwrap();
        // https://github.com/bevyengine/bevy/issues/1700
        input_state.reset(KeyCode::Space);
    }
}
