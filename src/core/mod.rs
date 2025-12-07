use wasabi_traits::scene::SceneFn;
use gethand::{DebugIf, Getters};
use std::fmt::{Debug, Formatter};
use crate::default::SceneManager as DefSceneManager;
use wasabi_traits::scene::SceneCatalog;

#[derive(Getters, DebugIf)]
pub struct Engine<ResourceSystem, SceneManager = DefSceneManager>
where
    SceneManager: SceneCatalog,
{
    scene_manager: SceneManager,
    resource_system: ResourceSystem
}
impl<SceneManager, ResourceSystem> Engine<ResourceSystem, SceneManager>
where
    SceneManager: SceneCatalog,
{
    pub fn new(scene_manager: SceneManager, resource_system: ResourceSystem) -> Self {
        Engine {
            scene_manager,
            resource_system
        }
    }

    pub fn next_scene(&mut self) {
        self.scene_manager.next()
    }

    // Todo: This fn should be deleted
    pub fn cycle(&mut self) {
        let scene = self.scene_manager.curr();
        for i in 0..scene.len() {
            scene.get(i).update();
            scene.get(i).render();
        }
    }
}


// impl Default for Engine {
//     fn default() -> Self {
//         Self {
//             scene_manager: DefSceneManager::default(),
//         }
//     }
// }
// 
