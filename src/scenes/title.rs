use bevy::prelude::*;
use bevy::reflect::Reflect;

#[derive(Reflect, Clone)]
pub struct TitleScreen {
    title: String,
    subtitle: String,
}

// Press space to hide TitleScreen and start the game
pub fn title_screen(
    input_state: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &TitleScreen, &mut Transform)>,
) {
    if input_state.just_pressed(KeyCode::Space) {
        // remove the title screen bundle
        // iterate over the results in the query
        if let Ok((entity, title_screen, transform)) = query.single_mut() {
            dbg!("beb");
            // remove the title screen
            commands.entity(entity).despawn();
            // move the paddle to the center of the screen
            //transform.set_translation_z(1.0);
        }
    }
}
