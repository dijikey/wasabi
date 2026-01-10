use std::sync::{Arc, Mutex, MutexGuard};

use gethand::{DebugIf, Getters};
use wasabi_traits::WindowContext;
use wasabi_traits::input::InputHandler;
use wasabi_traits::scene::SceneCatalog;

// #[allow(dead_code)]
// pub mod builder;
// pub use builder::builder;

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!($($arg)+)
    };
}

#[derive(Getters, DebugIf)]
pub struct Engine<EventSystem, SceneManager, Window>
where
    SceneManager: SceneCatalog,
    EventSystem: InputHandler,
    Window: WindowContext,
{
    scene_manager: SceneManager,
    #[skip]
    event_system: Arc<Mutex<EventSystem>>,
    window: Window,
}

impl<SceneManager, EventSystem, Window> Engine<EventSystem, SceneManager, Window>
where
    SceneManager: SceneCatalog,
    Window: WindowContext,
    EventSystem: InputHandler<Key = Window::Key, Action = Window::Action> + 'static,
{
    pub fn new(scene_manager: SceneManager, event_system: EventSystem, window: Window) -> Self {
        let mut engine = Engine {
            scene_manager,
            event_system: Arc::new(Mutex::new(event_system)),
            window,
        };

        let cloned = engine.event_system.clone();

        engine
            .window
            .hook_key_callback(move |key, action| match cloned.lock() {
                Ok(mut result) => result.key_callback(key, action),
                Err(mut err) => err.get_mut().key_callback(key, action),
            });

        info!("Engine initialized");
        engine
    }

    pub fn event_system(&mut self) -> MutexGuard<'_, EventSystem> {
        self.event_system.as_ref().lock().unwrap()
    }

    pub fn next_scene(&mut self) {
        self.scene_manager.next()
    }

    pub fn run<F: FnMut()>(&mut self, mut func: F) {
        while !self.window.should_close() {
            func();

            self.window.poll_events();
            self.window.swap_buffer();
        }
    }

    fn key_handler(&self, key: Window::Key, action: Window::Action) {}
}
