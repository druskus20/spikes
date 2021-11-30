use bevy::prelude::*;

use crate::game::GameState;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup.system()));
        //app.add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(input.system()));
        //app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(destroy.system()));
    }
}

struct TitleUI;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the title screen with spawn_bundle
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TitleUI)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Press Space".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLUE,
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
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::BLUE,
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
