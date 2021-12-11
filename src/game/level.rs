use crate::{screens::Screen, HEIGHT, WIDTH};
use bevy::prelude::*;

use super::{Collider, ColliderKind};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

struct Wall;
enum FacingTowards {
    Left,
    Right,
    Top,
    Bottom,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // TODO: handle materials in a proper way
    let wall_material = &materials.add(Color::rgb(0.8, 0.8, 0.8).into());

    spawn_wall(
        &mut commands,
        wall_material,
        Vec2::new(WIDTH / 2.0, 0.0),
        Vec2::new(0.0, HEIGHT),
        20.0,
    );

    spawn_wall(
        &mut commands,
        wall_material,
        Vec2::new(-(WIDTH / 2.0), 0.0),
        Vec2::new(0.0, HEIGHT),
        20.0,
    );

    spawn_wall(
        &mut commands,
        wall_material,
        Vec2::new(0.0, HEIGHT / 2.0),
        Vec2::new(HEIGHT, 0.0),
        20.0,
    );

    spawn_wall(
        &mut commands,
        wall_material,
        Vec2::new(0.0, -(HEIGHT / 2.0)),
        Vec2::new(HEIGHT, 0.0),
        20.0,
    );
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Wall)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Spawns a wall with a huge collider on the oposite side to "facing_side"
//
// |W| -------+
// |W|        |
// |W|        |
// |W|        |
// |W|        |
// |W| -------+
//
fn spawn_wall(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Vec2,
    size: Vec2,
    thickness: f32,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            sprite: Sprite::new(Vec2::new(size.x + thickness, size.y + thickness)),
            ..Default::default()
        })
        .insert(Collider {
            kind: ColliderKind::Wall,
            size: Vec2::new(size.x + thickness, size.y + thickness),
            position: Vec2::new(0.0, 0.0),
        })
        .insert(Wall);
}
