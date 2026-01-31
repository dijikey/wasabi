use std::{fmt::Debug, os::raw::c_void};

pub trait WindowContext {
    type Event: Debug + Clone + PartialEq;

    fn should_close(&self) -> bool;
    fn swap_buffer(&mut self);
    fn poll_events(&mut self);
    fn set_version(&mut self, major: u32, minor: u32);
    fn get_framebuffer_size(&self) -> (i32, i32);
    fn flush(&self) -> impl Iterator<Item = Self::Event>;

    fn loader_function(&mut self, s: &str) -> *const c_void;
}
