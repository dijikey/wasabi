use std::fmt::Debug;
use crate::core::scene::{SceneCatalog, SceneFn};

pub mod scene;

#[derive(Debug)] // todo: Remove is derive and make custom
pub struct Engine<SceneManager>
where SceneManager: SceneCatalog{
    scene_manager: SceneManager
}

impl <SceneManager> Engine<SceneManager>
where SceneManager: SceneCatalog{
    pub fn new(scene_manager: SceneManager) -> Engine<SceneManager> {
        Engine{
            scene_manager
        }
    }
    
    pub fn next_scene(&mut self) { self.scene_manager.next() }

    pub fn cycle(&mut self) {
        let scene = self.scene_manager.curr();
        for i in 0..scene.len() {
            scene.get(i).update();
            scene.get(i).render();
        }
    }
}


pub trait Layer: Debug {
    fn render(&mut self);

    fn update(&mut self);
}