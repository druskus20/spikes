use bevy::prelude::*;
use bevy::reflect::Reflect;

use super::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_system.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(player_movement_system.system())
                    .with_system(animate_sprite_system.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(despawn.system()));
    }
}

struct GameEntity;

pub struct Player {
    speed: f32,
}

pub enum ColliderKind {
    Solid,
    Spike,
    Player,
}

pub struct Spike {
    position: Vec3,
    color: Color,
    size: Vec3,
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // animate the sprite
            let current_index = sprite.index;
            let next_index = (current_index + 1) % 4 as u32;
            sprite.index = next_index;
            timer.reset();
        }
    }
}

// TODO: Clean this up
pub fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // player
    let texture_handle = asset_server.load("textures/rpg/chars/example.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 10, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(6.0, 6.0, 0.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
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
    // TODO: Use fromResources instead
    spawn_walls(commands, materials);
}

fn spawn_spike(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let spike = Spike {
        position: Vec3::new(0.0, 0.0, 0.0),
        size: Vec3::new(16.0, 16.0, 0.0),
        color: Color::rgb(0.5, 0.5, 0.5),
    };

    let spike_material = materials.add(spike.color.into());

    commands.spawn_bundle(SpriteBundle {
        material: spike_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(spike.size.x, spike.size.y)),
        ..Default::default()
    });
}

fn spawn_walls(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
