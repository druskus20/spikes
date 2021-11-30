use bevy::prelude::*;

use super::Screen;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

struct Wall;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // TODO: handle materials in a proper way
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 5.0;
    let bounds = Vec2::new(500.0, 500.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .insert(Wall);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
            sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .insert(Wall);
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Wall)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}
