use builder_helper::Builder;
use glfw::{WindowHint, WindowMode};

use crate::{Initialized, Window, WindowError};
use std::fmt::Debug;

#[derive(Builder, Debug)]
pub struct WindowBuilder<'a> {
    pub title: String,
    pub size: (u32, u32),
    pub window_mode: WindowMode<'a>,
    pub resizeable: bool,
    pub decorated: bool,
    pub raw_input: bool,
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

impl<'a> WindowBuilder<'a> {
    #[inline]
    pub fn new() -> Self {
        return Self::default();
    }

    #[inline]
    #[allow(private_interfaces)]
    pub fn build(self) -> Result<Window<Initialized>, WindowError> {
        let mut win = Window::new()?;
        win.window_hint(WindowHint::Resizable(self.resizeable));
        win.window_hint(WindowHint::Focused(self.focused));
        win.window_hint(WindowHint::Visible(self.visible));
        win.window_hint(WindowHint::Maximized(self.maximized));

        let mut win = win
            .init((self.size.0, self.size.1, &self.title, self.window_mode))
            .ok_or(WindowError::WindowNone)?;

        let raw = unsafe { win.raw.as_mut().unwrap_unchecked() };

        raw.set_decorated(self.decorated);
        raw.set_raw_mouse_motion(self.raw_input);

        Ok(win)
    }
}

impl<'a> Default for WindowBuilder<'a> {
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
            title: String::from("Application"),
            size: (800, 600),
            window_mode: WindowMode::Windowed,
            decorated: true,
            raw_input: false,
        }
    }
}
