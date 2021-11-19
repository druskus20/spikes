use bevy::prelude::*;
use bevy::reflect::Reflect;

use super::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_system.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(player_movement_system.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn.system()));
    }
}

struct GameEntity;

#[derive(Reflect, Clone)]
pub struct Player {
    speed: f32,
}

#[derive(Reflect, Clone)]
pub enum ColliderKind {
    Solid,
    Enemy,
    Player,
}

// TODO: Clean this up
pub fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .insert(Player { speed: 1000.0 })
        .insert(GameEntity)
        .insert(ColliderKind::Player);

    // scoreboard
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.5, 0.5, 1.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],

                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameEntity);

    spawn_walls(commands, materials, asset_server);
}

fn spawn_walls(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 900.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(ColliderKind::Solid);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(ColliderKind::Solid);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(ColliderKind::Solid);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(ColliderKind::Solid);
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    mut state: ResMut<State<GameState>>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        // TODO: remove this
        if keyboard_input.just_pressed(KeyCode::A) {
            state.set(GameState::GameOver).unwrap();
        }

        let (mut x, mut y) = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            y += 1.0;
        }
        let translation = &mut transform.translation;

        translation.x += time.delta_seconds() * x * player.speed;
        translation.y += time.delta_seconds() * y * player.speed;
        translation.x = translation.x.min(410.0).max(-410.0); // TODO: Check for collisions
        translation.y = translation.y.min(410.0).max(-410.0); // TODO: Check for collisions
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
