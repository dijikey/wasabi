//! This module provides main traits for preparing the engine.

use std::{fmt::Debug, hash::Hash};

pub mod input;
pub mod scene;

/// Trait for layers with render/update ordering support
pub trait Layer: Debug {
    /// Renders the layer
    ///
    /// This method is called each frame to draw the layer's content.
    /// Rendering should be idempotent and should not change the layer's state.
    fn render(&mut self);

    /// Whether the layer is visible
    fn is_visible(&self) -> bool {
        true
    }

    /// Updates the layer's state
    ///
    /// Called each frame before rendering. This should handle
    /// input processing, animations, game logic, and other state updates.
    fn update(&mut self);

    /// Whether the layer is active (gets updated)
    fn is_frozen(&self) -> bool {
        false
    }
}

pub trait WindowContext {
    type Key: Debug + Clone + Copy + Eq + Hash + PartialEq;
    type Action: Debug + Clone + Copy + Eq + Hash + PartialEq;

    fn should_close(&self) -> bool;
    fn swap_buffer(&mut self);
    fn poll_events(&mut self);

    fn hook_key_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Self::Key, Self::Action) + 'static;

    // fn loader_function<T>(&mut self, s: &str) -> *const T;
}
