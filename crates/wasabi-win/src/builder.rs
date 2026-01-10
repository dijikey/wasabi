use builder_helper::Builder;
use glfw::WindowHint;

use crate::{Constructed, Window, WindowError};
use std::fmt::Debug;

#[derive(Builder, Debug)]
pub struct WindowBuilder {
    pub resizeable: bool,
    // pub min_size: Option<(u32, u32)>,
    // pub max_size: Option<(u32, u32)>,
    pub maximized: bool,
    // pub minimized: bool,
    pub visible: bool,
    // pub transparent: bool,
    // pub any_thread: bool,
    // pub dpi_aware: bool,
    pub focused: bool,
}

impl WindowBuilder {
    #[inline]
    pub fn new() -> Self {
        return Self::default();
    }

    #[inline]
    #[allow(private_interfaces)]
    pub fn build(self) -> Result<Window<Constructed>, WindowError> {
        let mut win = Window::new()?;
        win.window_hint(WindowHint::Resizable(self.resizeable));
        win.window_hint(WindowHint::Focused(self.focused));
        win.window_hint(WindowHint::Visible(self.visible));
        win.window_hint(WindowHint::Maximized(self.maximized));

        Ok(win)
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            resizeable: true,
            // min_size: None,
            // max_size: None,
            maximized: false,
            // minimized: false,
            visible: true,
            // transparent: false,
            // dpi_aware: false,
            // any_thread: false,
            focused: true,
        }
    }
}
