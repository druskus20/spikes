use bevy::prelude::*;

use super::Screen;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(Screen::Game).with_system(setup.system()))
            .add_system_set(SystemSet::on_update(Screen::Game).with_system(movement.system()))
            .add_system_set(SystemSet::on_exit(Screen::Game).with_system(despawn.system()));
    }
}

pub struct Player {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
        .insert(Player { speed: 1000.0 });
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
        translation.x = translation.x.min(410.0).max(-410.0);
        translation.y = translation.y.min(410.0).max(-410.0);
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Player)>) {
    commands
        .entity(query.single().unwrap().0)
        .despawn_recursive();
}
