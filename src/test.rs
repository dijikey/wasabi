extern crate wasabi_win as win;

use crate::prelude::*;
use log::info;
use wasabi_event_sys::{EventSystem, Tag};
use wasabi_win::builder::WindowBuilder;

fn log_init() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .is_test(true)
        .try_init();
}

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

        let window = WindowBuilder::default()
            .build()
            .unwrap()
            .init((800, 600, "Application", win::WindowMode::Windowed))
            .unwrap();

        Engine::new(scene_manager, event_system, window)
    };

    let tag = Tag::KeyPressed(Box::new(move |key| {
        info!("{key:?}");
    }));

    engine.event_system().insert(tag);

    engine.run(|| {});

    println!("{engine:?}");
}
