use crate::core::scene::{SceneCatalog, SceneFn};
use gethand::Getters;
use std::fmt::{Debug, Formatter};

pub mod scene;
use scene::SceneManager as DefSceneManager;

#[derive(Getters)]
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

macro_rules! debug_impl {
    [$(trait_name: $trn: ident, r#trait: $tr:ident, field: $field:ident),+] => {
        impl<$($trn,)+> Debug for Engine<$($trn,)+>
            where $($trn: $tr + Debug)+{
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Engine")
                $( .field(stringify!($field), &self.$field ) )+
                .finish()
            }
        }
    };
}

debug_impl![
    trait_name: SC, r#trait: SceneCatalog, field: scene_manager
];

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
