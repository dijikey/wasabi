use crate::default::SceneManager as DefSceneManager;
use gethand::{DebugIf, Getters};
use std::fmt::{Debug, Formatter};
use wasabi_traits::event::EventHandler;
use wasabi_traits::scene::SceneCatalog;
use wasabi_traits::scene::SceneFn;

#[derive(Getters, DebugIf)]
pub struct Engine<EventSystem, SceneManager = DefSceneManager>
where
    SceneManager: SceneCatalog,
    EventSystem: EventHandler,
{
    scene_manager: SceneManager,
    event_system: EventSystem,
}
impl<SceneManager, EventSystem> Engine<EventSystem, SceneManager>
where
    SceneManager: SceneCatalog,
    EventSystem: EventHandler,
{
    pub fn new(scene_manager: SceneManager, event_system: EventSystem) -> Self {
        Engine {
            scene_manager,
            event_system,
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
