use std::sync::{Arc, Mutex, MutexGuard};

use gethand::{DebugIf, Getters};
use wasabi_traits::InputHandler;
use wasabi_traits::Renderer as RendererTrait;
use wasabi_traits::WindowContext;
use wasabi_traits::scene::SceneCatalog;

// #[allow(dead_code)]
// pub mod builder;
// pub use builder::builder;

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!(target: "Engine::Core", $($arg)+)
    };
}

#[derive(Getters, DebugIf)]
pub struct Engine<EventSystem, SceneManager, Window, Renderer>
where
    SceneManager: SceneCatalog,
    EventSystem: InputHandler,
    Window: WindowContext,
    Renderer: RendererTrait,
{
    scene_manager: SceneManager,
    #[skip]
    event_system: Arc<Mutex<EventSystem>>,
    window: Window,
    renderer: Renderer,
}

impl<SceneManager, EventSystem, Window, Renderer>
    Engine<EventSystem, SceneManager, Window, Renderer>
where
    SceneManager: SceneCatalog,
    Window: WindowContext,
    Renderer: RendererTrait,
    EventSystem: InputHandler<
            Key: From<Window::Key>,
            Action: From<Window::Action>,
            MouseKey: From<Window::MouseKey>,
        > + 'static,
{
    pub fn new(
        scene_manager: SceneManager,
        event_system: EventSystem,
        mut window: Window,
        renderer: Option<Renderer>,
    ) -> Self {
        let renderer = match renderer {
            Some(mut value) => {
                value.with_context(|s| window.loader_function(s));
                value
            }
            None => Renderer::new(|s| window.loader_function(s)),
        };

        let mut engine = Engine {
            scene_manager,
            event_system: Arc::new(Mutex::new(event_system)),
            window,
            renderer,
        };

        engine.hook_all();

        info!("Engine initialized");
        engine
    }

    pub fn event_system(&mut self) -> MutexGuard<'_, EventSystem> {
        self.event_system.as_ref().lock().unwrap()
    }

    pub fn next_scene(&mut self) {
        self.scene_manager.next()
    }

    pub fn run<F: FnMut(&mut Self)>(&mut self, mut func: F) {
        while !self.window.should_close() {
            self.renderer.clear();
            func(self);

            self.window.poll_events();
            self.window.swap_buffer();
        }
    }

    fn hook_all(&mut self) {
        let win = &mut self.window;

        let cloned = self.event_system.clone();
        win.hook_key_callback(move |key, action| match cloned.lock() {
            Ok(mut result) => result.key_callback(key.into(), action.into()),
            Err(mut err) => err.get_mut().key_callback(key.into(), action.into()),
        });

        let cloned = self.event_system.clone();
        win.hook_mouse_position_callback(move |x, y| match cloned.lock() {
            Ok(mut result) => result.mouse_moved(x, y),
            Err(mut err) => err.get_mut().mouse_moved(x, y),
        });

        let cloned = self.event_system.clone();
        win.hook_mouse_callback(move |key, action| match cloned.lock() {
            Ok(mut result) => result.mouse_callback(key.into(), action.into()),
            Err(mut err) => err.get_mut().mouse_callback(key.into(), action.into()),
        });
    }
}
