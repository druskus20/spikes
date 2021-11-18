use bevy::{prelude::*, reflect::TypeRegistry};

pub mod game;
pub mod title;

pub fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/load_scene_example.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    asset_server.watch_for_changes().unwrap();
}

pub fn save_scene_system(world: &mut World) {
    dbg!("save");
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&world, &type_registry);
    println!("{}", scene.serialize_ron(&type_registry).unwrap());
    // TODO: save scene
}
