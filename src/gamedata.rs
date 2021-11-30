use bevy::prelude::*;

pub struct GameDataPlugin;
impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Score { score: -1 });
    }
}

struct Score {
    score: i32,
}
