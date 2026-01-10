extern crate wasabi_win as win;
use std::fmt::Debug;

use crate::prelude::*;
use log::info;
use wasabi_event_sys::{EventSystem, Key, Tag};
use wasabi_renderer::color::Color;

fn log_init() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

#[derive(Debug)]
struct Vector2<T: Debug> {
    x: T,
    y: T,
}

static mut MOUSE_POS: Vector2<f64> = Vector2 { x: 0.0, y: 0.0 };

#[test]
pub fn main() {
    #[derive(Debug)]
    struct Controller {
        screen: Screen,
    }

    impl SceneCatalog for Controller {
        type Scene = Screen;

        fn next(&mut self) {}

        fn curr(&mut self) -> &mut Self::Scene {
            &mut self.screen
        }
    }

    #[derive(Debug)]
    struct Screen {
        layer: Box<dyn Layer>,
    }

    impl SceneFn for Screen {
        fn get(&mut self, _: usize) -> &mut Box<dyn Layer> {
            &mut self.layer
        }

        fn len(&self) -> usize {
            1
        }
    }

    #[derive(Debug)]
    struct Alpha {}

    impl Layer for Alpha {
        fn render(&mut self) {
            println!("render");
        }

        fn update(&mut self) {
            println!("update");
        }
    }

    log_init();

    let mut engine = {
        let scene_manager = Controller {
            screen: Screen {
                layer: Box::new(Alpha {}),
            },
        };

        let event_system = EventSystem::default();

        let window = win::builder()
            .with_decorated(true)
            .with_raw_input(true)
            .build()
            .unwrap();

        Engine::new(
            scene_manager,
            event_system,
            window,
            Option::<wasabi_renderer::Renderer>::None,
        )
    };

    let key_traker = Tag::KeyCallback(Box::new(move |key, action| {
        match key {
            #[allow(static_mut_refs)]
            Key::V => unsafe { info!("Mouse position {MOUSE_POS:?}") },
            _ => {}
        }
        info!("Keyboard {key:?} : {action:?}");
    }));

    let mouse_tracker = Tag::MouseMoved(Box::new(move |x, y| unsafe {
        MOUSE_POS.x = x;
        MOUSE_POS.y = y;
    }));

    let mouse_callback = Tag::MouseCallback(Box::new(move |key, action| {
        info!("Mouse {key:?} : {action:?}");
    }));

    engine.event_system().insert(key_traker);
    engine.event_system().insert(mouse_tracker);
    engine.event_system().insert(mouse_callback);

    engine.run(|engine| {
        let time = engine.window().time();
        let sin = time.sin();
        engine
            .renderer_mut()
            .set_color(Color::from_f32(sin as f32, 0.5, 0.5, 1.0));
    });

    println!("{engine:?}");
}
