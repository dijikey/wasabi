use crate::prelude::*;

#[test]
pub fn new() {
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

    let mut engine = Engine::new(
        Controller {
            screen: Screen {
                layer: Box::new(Alpha {}),
            },
        },
        String::from("Resource System is any struct|enum"),
    );

    println!("engine created");
    println!("{engine:?}");

    engine.cycle()
}
