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
            text: Text::with_section(
                "Spikes".to_string(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
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
