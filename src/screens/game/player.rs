use std::time::SystemTime;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

use super::Collider;
use super::ColliderKind;
use super::Screen;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(Screen::Game)
                    .with_system(collision.system().label("collision"))
                    .with_system(movement.system().after("collision")),
            )
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

#[derive(Debug)]
pub struct Player {
    speed: f32,
    colliding_directions: CollidingDirections,
}

#[derive(Debug, Default)]
struct CollidingDirections {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // player
    let texture_handle = asset_server.load("textures/rpg/chars/example.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 10, 10);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //commands
    //    .spawn_bundle(SpriteSheetBundle {
    //        texture_atlas: texture_atlas_handle,
    //        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //        ..Default::default()
    //    })
    //    .insert(Collider {
    //        kind: ColliderKind::Player,
    //        size: Vec2::new(100.0, 100.0),
    //    })
    //    .insert(Player { speed: 100.0 });

    let wall_material = materials.add(Color::rgb(0.8, 0.1, 0.1).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            //transform: Transform::from_xyz(position.x, position.y, 0.0),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Collider {
            kind: ColliderKind::Player,
            size: Vec2::new(32.0, 32.0),
        })
        .insert(Player {
            speed: 500.0,
            colliding_directions: Default::default(),
        });
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
        let (mut x, mut y) = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) && !player.colliding_directions.left {
            x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) && !player.colliding_directions.right {
            x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) && !player.colliding_directions.down {
            y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) && !player.colliding_directions.up {
            y += 1.0;
        }
        let translation = &mut transform.translation;

        translation.x += time.delta_seconds() * x * player.speed;
        translation.y += time.delta_seconds() * y * player.speed;
        //translation.x = translation.x.min(410.0).max(-410.0);
        //translation.y = translation.y.min(410.0).max(-410.0);
    }
}

// TODO: Clean this up after implementing spikes
fn collision(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Player, &mut Handle<ColorMaterial>)>,
    player_collider_query: Query<(&Collider, &Transform), With<Player>>,
    collider_query: Query<(&Collider, &Transform), Without<Player>>,
) {
    let mut player_color = Color::rgb(0.0, 0.0, 1.0);
    let mut colliding_directions = CollidingDirections::default();

    if let Ok((player_collider, player_transform)) = player_collider_query.single() {
        for (collider, transform) in collider_query.iter() {
            if let Some(collision) = collide(
                transform.translation,
                collider.size,
                player_transform.translation,
                player_collider.size,
            ) {
                player_color = Color::rgb(0.0, 1.0, 0.0);
                match collision {
                    Collision::Left => colliding_directions.left = true,
                    Collision::Right => colliding_directions.right = true,
                    Collision::Top => colliding_directions.up = true,
                    Collision::Bottom => colliding_directions.down = true,
                }
            }
        }
    }
    if let Ok((mut player, material_handle)) = query.single_mut() {
        player.colliding_directions = colliding_directions;
        if let Some(color_mat) = materials.get_mut(material_handle.id) {
            color_mat.color = player_color;
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Player)>) {
    commands
        .entity(query.single().unwrap().0)
        .despawn_recursive();
}
