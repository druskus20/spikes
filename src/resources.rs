use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GlobalResources>();
    }
}

pub struct GlobalResources {
    pub none: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>,
    pub font: Handle<Font>,
}

impl FromWorld for GlobalResources {
    fn from_world(world: &mut World) -> Self {
        // TODO: come up with a better way to do this
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let none = materials.add(Color::rgb(0.0, 0.0, 0.0).into());
        let background = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

        let asset_server = world.get_resource::<AssetServer>().unwrap();
        GlobalResources {
            none,
            background,
            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        }
    }
}
