use bevy::{prelude::*, reflect::TypeRegistry};

pub mod game;
pub mod title;

pub struct ScenesPlugin;
impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_system(load_scene_system.system())
            //.add_startup_system(save_scene_system.exclusive_system())
            .add_state(GameState::Title)
            .add_plugin(game::GamePlugin)
            .add_plugin(title::TitlePlugin);
    }
}

#[derive(Reflect, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    Title,
    Game,
    GameOver,
}

//pub fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
//    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/load_scene_example.scn.ron");
//    scene_spawner.spawn_dynamic(scene_handle);
//    asset_server.watch_for_changes().unwrap();
//}
//
//pub fn save_scene_system(world: &mut World) {
//    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
//    let scene = DynamicScene::from_world(&world, &type_registry);
//    println!("{}", scene.serialize_ron(&type_registry).unwrap());
//    // TODO: save scene
//}
