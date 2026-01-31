use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum EventCallback {
    None,
    FramebufferResize(i32, i32),
}

pub trait InputHandler {
    type Event: Debug + Clone + PartialEq;

    fn handle(&mut self, event: Self::Event) -> EventCallback;
    fn buffer_clear(&mut self) {}
}
