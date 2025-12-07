use crate::core::scene::{SceneCatalog, SceneFn};
use gethand::Getters;
use std::fmt::{Debug, Formatter};

#[derive(Getters, DebugIf)]
pub struct Engine<SceneManager = DefSceneManager>
where
    SceneManager: SceneCatalog,
{
    scene_manager: SceneManager,
}
impl<SceneManager> Engine<SceneManager>
where
    SceneManager: SceneCatalog,
{
    pub fn new(scene_manager: SceneManager) -> Engine<SceneManager> {
        Engine { scene_manager }
    }

    pub fn next_scene(&mut self) {
        self.scene_manager.next()
    }

    pub fn cycle(&mut self) {
        let scene = self.scene_manager.curr();
        for i in 0..scene.len() {
            scene.get(i).update();
            scene.get(i).render();
        }
    }
}



impl Default for Engine {
    fn default() -> Self {
        Self {
            scene_manager: DefSceneManager::default(),
        }
    }
}

pub trait Layer: Debug {
    fn render(&mut self);

    fn update(&mut self);
}
