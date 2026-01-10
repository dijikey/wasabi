use std::{fmt::Debug, hash::Hash, os::raw::c_void};

pub trait WindowContext {
    type MouseKey: Debug + Clone + Copy + Eq + Hash + PartialEq;
    type Key: Debug + Clone + Copy + Eq + Hash + PartialEq;
    type Action: Debug + Clone + Copy + Eq + Hash + PartialEq;

    fn should_close(&self) -> bool;
    fn swap_buffer(&mut self);
    fn poll_events(&mut self);

    fn hook_key_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Self::Key, Self::Action) + 'static;

    fn hook_mouse_position_callback<F>(&mut self, callback: F)
    where
        F: FnMut(f64, f64) + 'static;

    fn hook_mouse_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Self::MouseKey, Self::Action) + 'static;

    fn loader_function(&mut self, s: &str) -> *const c_void;
}
