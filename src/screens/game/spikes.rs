use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, prelude::*};
use rand::Rng;
use std::fmt::Debug;

use anyhow::{bail, Result};

use crate::{HEIGHT, WIDTH};

use super::{Collider, ColliderKind, Screen};

const TIMESTEP: f64 = 15.0 / 60.0;

pub struct SpikesPlugin;
impl Plugin for SpikesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // TODO: Actually clean this up
        let custom_criteria = || {
            FixedTimestep::step(TIMESTEP).chain(
                (|In(input): In<ShouldRun>, state: Res<State<Screen>>| {
                    if state.current() == &Screen::Game {
                        input
                    } else {
                        ShouldRun::No
                    }
                })
                .system(),
            )
        };

        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_update(Screen::Game)
                    .with_system(move_spikes.system())
                    .with_system(remove_spikes.system()),
            )
            //            .add_system_set(
            //                SystemSet::new().with_system(
            //                    generate_new_spikes
            //                        .system()
            //                        .with_run_criteria(combined_criteria),
            //                ),
            //            )
            .add_system_set(
                SystemSet::new().with_system(
                    generate_new_spikes
                        .system()
                        .with_run_criteria(custom_criteria()),
                ),
            )
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

fn setup() {}

//fn combined_criteria(
//    c1: impl System<Out = ShouldRun>,
//    c2: impl System<Out = ShouldRun>,
//    f: impl Fn(ShouldRun, ShouldRun) -> ShouldRun,
//) -> ShouldRun {
//    todo!()
//}

fn despawn(mut commands: Commands, query: Query<Entity, With<Spike>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Debug, Clone)]
struct Spike {
    direction: Direction,
    pub speed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl TryFrom<usize> for Direction {
    type Error = anyhow::Error;
    fn try_from(n: usize) -> Result<Self> {
        Ok(match n {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Top,
            3 => Direction::Bottom,
            _ => bail!("Invalid side"),
        })
    }
}

impl From<&Direction> for Vec2 {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Top => Vec2::new(0.0, -1.0),
            Direction::Bottom => Vec2::new(0.0, 1.0),
        }
    }
}

fn generate_new_spikes(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(-WIDTH / 2.0..WIDTH / 2.0);
    let direction = Direction::try_from(rng.gen_range(0..4)).unwrap();

    let (position, size) = match direction {
        Direction::Bottom => (Vec2::new(length, -HEIGHT / 2.0), Vec2::new(5.0, 50.0)),
        Direction::Top => (Vec2::new(length, HEIGHT / 2.0), Vec2::new(5.0, 50.0)),
        Direction::Right => (Vec2::new(-WIDTH / 2.0, length), Vec2::new(50.0, 5.0)),
        Direction::Left => (Vec2::new(WIDTH / 2.0, length), Vec2::new(50.0, 5.0)),
    };

    let material = &materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    spawn_spike(
        &mut commands,
        material,
        position,
        Vec2::new(0.0, 0.0),
        size,
        10.0,
        direction,
        5.0,
    )
}

fn move_spikes(mut query: Query<(&mut Spike, &mut Transform)>) {
    for (spike, mut transform) in query.iter_mut() {
        transform.translation += Vec3::from((Vec2::from(&spike.direction) * spike.speed, 0.0));
    }
}

fn remove_spikes(mut commands: Commands, query: Query<(Entity, &Spike, &Transform)>) {
    for (entity, spike, transform) in query.iter() {
        let position = transform.translation;
        let direction = spike.direction.clone();

        if direction == Direction::Left && position.x < -WIDTH / 2.0
            || direction == Direction::Right && position.x > WIDTH / 2.0
            || direction == Direction::Top && position.y < -HEIGHT / 2.0
            || direction == Direction::Bottom && position.y > HEIGHT / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_spike(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Vec2,
    rotation: Vec2,
    size: Vec2,
    thickness: f32,
    direction: Direction,
    speed: f32,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            transform: Transform::from_xyz(position.x, position.y, 0.0),
            sprite: Sprite::new(Vec2::new(size.x + thickness, size.y + thickness)),
            ..Default::default()
        })
        .insert(Spike { direction, speed })
        .insert(Collider {
            kind: ColliderKind::Spike,
            size: Vec2::new(size.x + thickness, size.y + thickness),
            ..Default::default()
        });
}
