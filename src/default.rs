//! This module contains the basic trait implementation.
//!
//! Everything contained here is not suitable for game development (most likely)

use std::fmt::Debug;
use wasabi_traits::Layer;
use wasabi_traits::scene::{SceneCatalog, SceneFn};

/// Is not an implementation for creating games, mostly just for examples.
///
/// A scene containing multiple renderable layers
///
/// To create your own structure, use a trait [`SceneFn`]
#[derive(Debug, Default)]
pub struct Scene {
    layers: Vec<Box<dyn Layer>>,
}

impl Scene {
    /// Creates a new empty scene
    pub fn new() -> Scene {
        Scene { layers: vec![] }
    }

    pub fn with_layers(layers: Vec<Box<dyn Layer>>) -> Scene {
        Scene { layers }
    }

    /// Adds a new layer to the scene
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }
}

impl SceneFn for Scene {
    fn get(&mut self, i: usize) -> &mut Box<dyn Layer> {
        unsafe { self.layers.get_unchecked_mut(i) }
    }

    fn len(&self) -> usize {
        self.layers.len()
    }
}

/// Is not an implementation for creating games, mostly just for examples.
///
/// To create your own structure, use a trait [`SceneCatalog`]
#[derive(Debug, Default)]
pub struct SceneManager {
    pub scenes: Vec<Scene>,
    pub curr_index: usize,
}

impl SceneManager {
    /// Creates a new SceneManager with empty scene list
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            curr_index: 0,
        }
    }

    /// Push a scene to the manager
    pub fn push_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }
}

impl SceneCatalog for SceneManager {
    type Scene = Scene;

    fn next(&mut self) {
        match self.curr_index == self.scenes.len() - 1 {
            true => self.curr_index = 0,
            false => self.curr_index += 1,
        };

        self.curr().on_awake()
    }

    fn curr(&mut self) -> &mut Self::Scene {
        unsafe { self.scenes.get_unchecked_mut(0) }
    }
}
