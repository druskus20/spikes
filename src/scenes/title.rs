use bevy::prelude::*;
use bevy::reflect::Reflect;

use crate::scenes::GameState;

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Title).with_system(setup_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Title).with_system(handle_input_system.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Title).with_system(despawn.system()));
    }
}

#[derive(Reflect, Clone)]
pub struct TitleScreen {}

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Spawn the title screen with spawn_bundle
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Spikes".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            color: Color::WHITE,
                            font_size: 64.0,
                        },
                    },
                    TextSection {
                        value: "Press SPACE to Start".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            color: Color::WHITE,
                            font_size: 32.0,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TitleScreen {});
}

pub fn handle_input_system(input_state: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input_state.just_pressed(KeyCode::Space) {
        state.set(GameState::Game).unwrap();
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &TitleScreen)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
