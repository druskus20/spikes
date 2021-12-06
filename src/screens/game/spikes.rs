use bevy::{core::FixedTimestep, prelude::*};
use rand::Rng;

use anyhow::{bail, Result};

use crate::{HEIGHT, WIDTH};

use super::Screen;

pub struct SpikesPlugin;
impl Plugin for SpikesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(Screen::Game).with_system(generate_spikes.system()),
            )
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

fn setup() {}
fn despawn() {}

struct Spike {}

pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl TryFrom<usize> for Side {
    type Error = anyhow::Error;
    fn try_from(n: usize) -> Result<Self> {
        Ok(match n {
            0 => Side::Left,
            1 => Side::Right,
            2 => Side::Top,
            3 => Side::Bottom,
            _ => bail!("Invalid side"),
        })
    }
}

// Generate a new random position for a spike, making sure that it doesnt overlapp with any other spikes
fn generate_spike_position() -> Result<Vec2> {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(-WIDTH / 2.0..WIDTH / 2.0);
    let side = Side::try_from(rng.gen_range(0..4))?;

    Ok(match side {
        Side::Left => Vec2::new(length, -WIDTH / 2.0),
        Side::Right => Vec2::new(length, WIDTH / 2.0),
        Side::Top => Vec2::new(-WIDTH / 2.0, length),
        Side::Bottom => Vec2::new(WIDTH / 2.0, length),
    })
}

fn generate_spikes(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material = &materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    spawn_spike(
        &mut commands,
        material,
        generate_spike_position().unwrap(),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        10.0,
    )
}

fn spawn_spike(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Vec2,
    rotation: Vec2,
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
        //.insert(Collider {
        //    kind: ColliderKind::Wall,
        //    size: Vec2::new(
        //        size.x + thickness + (2.0 * collider_offset),
        //        size.y + thickness + (2.0 * collider_offset),
        //    ),
        //    position: collider_position,
        //})
        .insert(Spike {});
}
