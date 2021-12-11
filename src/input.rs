/*
 * This module should bind inputs to PlayerActions (events)
 * and it should handle input for different scenes
 */

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
pub struct InputPlugin(InputBindings);

impl InputPlugin {
    pub fn new() -> Self {
        InputPlugin(InputBindings::default())
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(update_active_bindings.system())
            .init_resource::<InputBindings>();
    }
}

fn update_active_bindings(
    keyboard_input: Res<Input<KeyCode>>,
    mut mappings: ResMut<InputBindings>,
) {
    mappings.update(&keyboard_input);
}

pub struct InputBindings {
    mappings: HashMap<InputAction, PlayerActions>,
    active: HashSet<PlayerActions>,
}

impl InputBindings {
    fn update(&mut self, input: &Input<KeyCode>) {
        self.active.clear();
        for (input_action, player_action) in self.mappings.iter() {
            if input_action.is_active(input) {
                self.active.insert(*player_action);
            }
        }
    }

    pub fn is_active(&self, player_action: &PlayerActions) -> bool {
        self.active.contains(player_action)
    }
}

impl Default for InputBindings {
    fn default() -> Self {
        InputBindings {
            mappings: HashMap::from_iter([
                (
                    InputAction::new(KeyCode::W, InputPhase::Pressed),
                    PlayerActions::MoveUp,
                ),
                (
                    InputAction::new(KeyCode::S, InputPhase::Pressed),
                    PlayerActions::MoveDown,
                ),
                (
                    InputAction::new(KeyCode::A, InputPhase::Pressed),
                    PlayerActions::MoveLeft,
                ),
                (
                    InputAction::new(KeyCode::D, InputPhase::Pressed),
                    PlayerActions::MoveRight,
                ),
            ]),
            active: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputAction {
    pub key_code: KeyCode,
    pub input_phase: InputPhase,
}

impl InputAction {
    pub fn new(key_code: KeyCode, input_phase: InputPhase) -> Self {
        InputAction {
            key_code,
            input_phase,
        }
    }

    fn is_active(&self, input: &Input<KeyCode>) -> bool {
        match self.input_phase {
            InputPhase::Pressed => input.pressed(self.key_code),
            InputPhase::JustPressed => input.just_pressed(self.key_code),
            InputPhase::JustReleased => input.just_released(self.key_code),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum InputPhase {
    JustPressed,
    Pressed,
    JustReleased,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum PlayerActions {
    None,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Default for PlayerActions {
    fn default() -> Self {
        PlayerActions::None
    }
}
