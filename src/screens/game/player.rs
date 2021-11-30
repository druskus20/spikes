use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use super::Collider;
use super::ColliderKind;
use super::Screen;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(Screen::Game)
                    .with_system(movement.system())
                    .with_system(collision.system()),
            )
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

#[derive(Debug)]
pub struct Player {
    speed: f32,
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

    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            //transform: Transform::from_xyz(position.x, position.y, 0.0),
            sprite: Sprite::new(Vec2::new(16.0, 16.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Collider {
            kind: ColliderKind::Player,
            size: Vec2::new(16.0, 16.0),
        })
        .insert(Player { speed: 100.0 });
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if let Ok((player, mut transform)) = query.single_mut() {
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
        //translation.x = translation.x.min(410.0).max(-410.0);
        //translation.y = translation.y.min(410.0).max(-410.0);
    }
}

fn collision(
    player_query: Query<(&Collider, &Transform), With<Player>>,
    collider_query: Query<(&Collider, &Transform), Without<Player>>,
) {
    if let Ok((player_collider, player_transform)) = player_query.single() {
        for (collider, transform) in collider_query.iter() {
            if collide(
                player_transform.translation,
                player_collider.size,
                transform.translation,
                collider.size,
            )
            .is_some()
            {
                dbg!("Collision");
            }
        }
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Player)>) {
    commands
        .entity(query.single().unwrap().0)
        .despawn_recursive();
}
