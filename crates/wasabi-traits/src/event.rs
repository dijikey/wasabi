pub trait EventHandler {
    /// You don't need to call the function yourself, it's called inside the engine.
    fn key_pressed(&mut self);
    /// You don't need to call the function yourself, it's called inside the engine.
    fn key_released(&mut self);
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
