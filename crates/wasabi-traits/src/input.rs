use std::{fmt::Debug, hash::Hash};

pub trait InputHandler {
    type Key: Debug + Clone + Copy + Eq + Hash + PartialEq;
    type Action: Debug + Clone + Copy + Eq + Hash + PartialEq;

    /// You don't need to call the function yourself, it's called inside the engine.
    fn key_callback(&mut self, key: Self::Key, action: Self::Action);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn mouse_moved(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn mouse_pressed(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn mouse_released(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn window_closed(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn window_resized(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn window_focused(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn window_lost_focus(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn window_moved(&mut self);
}
