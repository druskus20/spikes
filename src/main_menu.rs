use bevy::prelude::*;

use crate::{gamedata::GameState, resources::GlobalResources};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu).with_system(menu_input.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn.system()));
        //app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(input.system()));
        //app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy.system()));
    }
}

struct MainMenuUI;

fn setup(mut commands: Commands, global_resources: ResMut<GlobalResources>) {
    // TODO: Figure out a good way to handle colors
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: global_resources.background.clone(),
            ..Default::default()
        })
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Press Space".to_string(),
                    TextStyle {
                        font: global_resources.font.clone(),
                        font_size: 30.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(90.0)),
                    margin: Rect::all(Val::Auto),
                    padding: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Spikes".to_string(),
                    TextStyle {
                        font: global_resources.font.clone(),
                        font_size: 60.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                    TextAlignment::default(),
                ),
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(90.0)),
                    margin: Rect::all(Val::Auto),
                    padding: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn menu_input(input_state: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if input_state.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game).unwrap();
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &MainMenuUI)>) {
    commands
        .entity(query.single().unwrap().0)
        .despawn_recursive();
}
