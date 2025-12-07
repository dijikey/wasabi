use crate::Layer;

/// A catalog/traversal interface for managing a sequence of scenes
pub trait SceneCatalog {
    /// The concrete scene type managed by this catalog
    ///
    /// Must implement the SceneFn trait for layer access
    type Scene: SceneFn;

    /// Move to the next scene (wraps around)
    ///
    /// You should also call function [SceneFn::on_awake], because trait [SceneFn] expects this.
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

    /// Called when the [SceneFn] is first used
    fn on_awake(&mut self) {}

    /// Update the scene state
    ///
    /// Can be used for ECS system
    fn update(&mut self) {}
}
