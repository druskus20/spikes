use bevy::{prelude::*, reflect::TypeRegistry};

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(crate::title::TitlePlugin);
    }
}
