use core::time;

use bevy::prelude::*;

use crate::resources::GlobalResources;

use super::CurrentScreen;

pub struct EndScreenPlugin;

impl Plugin for EndScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(CurrentScreen::EndScreen).with_system(setup.system()),
        )
        .add_system_set(SystemSet::on_update(CurrentScreen::EndScreen).with_system(input.system()))
        .add_system_set(SystemSet::on_exit(CurrentScreen::EndScreen).with_system(despawn.system()));
    }
}

struct EndScreenUI;

fn setup(mut commands: Commands, global_resources: ResMut<GlobalResources>) {
    dbg!("End Screen Setup");
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
        .insert(EndScreenUI)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "The end!".to_string(),
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

fn input(mut input_state: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<CurrentScreen>>) {
    if input_state.just_pressed(KeyCode::Space) {
        game_state.set(CurrentScreen::MainMenu).unwrap();
        // https://github.com/bevyengine/bevy/issues/1700
        input_state.reset(KeyCode::Space);
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &EndScreenUI)>) {
    commands
        .entity(query.single().unwrap().0)
        .despawn_recursive();
}
