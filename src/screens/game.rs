use core::time;

use bevy::prelude::*;

use super::CurrentScreen;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(CurrentScreen::Game).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(CurrentScreen::Game).with_system(input.system()))
            .add_system_set(SystemSet::on_exit(CurrentScreen::Game).with_system(despawn.system()));
    }
}

fn setup() {
    dbg!("Game Setup");
}
fn despawn() {}

fn input(mut input_state: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<CurrentScreen>>) {
    if input_state.just_pressed(KeyCode::Space) {
        game_state.set(CurrentScreen::EndScreen).unwrap();
        // https://github.com/bevyengine/bevy/issues/1700
        input_state.reset(KeyCode::Space);
    }
}
