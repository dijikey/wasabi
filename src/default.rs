use crate::Layer;

/// A catalog/traversal interface for managing a sequence of scenes
pub trait SceneCatalog {
    /// The concrete scene type managed by this catalog
    ///
    /// Must implement the SceneFn trait for layer access
    type Scene: SceneFn;

    /// Move to the next scene (wraps around)
    fn next(&mut self);
    /// Get mutable access to current scene
    #[must_use]
    fn curr(&mut self) -> &mut Self::Scene;
}

/// Trait for scene layer management with indexed access
///
/// Similar to an indexable collection with dynamic dispatch
#[allow(clippy::len_without_is_empty)]
pub trait SceneFn {
    /// Returns a mutable reference to the i-th layer (0-based index).
    ///
    /// Caller must ensure: 0 ≤ i < self.count()
    #[must_use]
    fn get(&mut self, i: usize) -> &mut Box<dyn Layer>;
    /// Returns the total number of layers
    #[must_use]
    fn len(&self) -> usize;
}

/// A scene containing multiple renderable layers
///
/// Default realization for SceneFn
#[derive(Debug, Default)]
pub struct Scene {
    layers: Vec<Box<dyn Layer>>,
}

impl Scene {
    /// Creates a new empty scene
    pub fn new() -> Scene {
        Scene { layers: vec![] }
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

    /// Adds a scene to the manager
    pub fn add_scene(&mut self, scene: Scene) {
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
