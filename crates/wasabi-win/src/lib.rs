use std::os;

use glfw::{Context, PWindow};
use glfw::{Glfw, InitError};
use wasabi_traits::WindowContext;

pub use glfw::OpenGlProfileHint;
pub use glfw::WindowHint;
pub use glfw::WindowMode;

#[allow(dead_code)]
pub mod builder;
use crate::builder::WindowBuilder;

pub fn builder<'a>() -> WindowBuilder<'a> {
    WindowBuilder::default()
}

#[allow(private_bounds)]
#[derive(Debug)]
pub struct Window<Flag: WindowFlag> {
    pub glfw: Glfw,
    // SAFETY
    pub raw: Option<glfw::PWindow>,
    marker: std::marker::PhantomData<Flag>,
}

impl Window<Constructed> {
    pub fn new() -> Result<Self, WindowError> {
        use glfw::fail_on_errors;
        let glfw = glfw::init(fail_on_errors!())?;

        Ok(Window {
            glfw,
            raw: None,
            marker: std::marker::PhantomData,
        })
    }

    #[allow(private_interfaces)]
    pub fn init(mut self, attrs: (u32, u32, &str, WindowMode)) -> Option<Window<Initialized>> {
        let w = attrs.0;
        let h = attrs.1;
        let title = attrs.2;
        let mode = attrs.3;

        let (mut window, _) = match self.glfw.create_window(w, h, title, mode) {
            Some(v) => v,
            None => return None,
        };

        window.set_key_polling(true);

        // window.set_key_callback();

        Some(Window {
            glfw: self.glfw,
            raw: Some(window),
            marker: std::marker::PhantomData,
        })
    }

    pub fn window_hint(&mut self, hint: WindowHint) {
        self.glfw.window_hint(hint);
    }
}

impl Window<Initialized> {
    #[inline(always)]
    fn raw_mut(&mut self) -> &mut PWindow {
        // SAFETY
        unsafe { self.raw.as_mut().unwrap_unchecked() }
    }

    #[inline(always)]
    fn raw_ref(&self) -> &PWindow {
        // SAFETY
        unsafe { self.raw.as_ref().unwrap_unchecked() }
    }

    pub fn time(&self) -> f64 {
        self.glfw.get_time()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WindowError {
    AlreadyInitialized,
    Internal,
    WindowNone,
}

impl From<InitError> for WindowError {
    fn from(value: InitError) -> Self {
        match value {
            InitError::AlreadyInitialized => Self::AlreadyInitialized,
            InitError::Internal => Self::Internal,
        }
    }
}

impl WindowContext for Window<Initialized> {
    type MouseKey = glfw::MouseButton;
    type Key = glfw::Key;
    type Action = glfw::Action;

    fn should_close(&self) -> bool {
        self.raw_ref().should_close()
    }

    fn swap_buffer(&mut self) {
        self.raw_mut().swap_buffers();
    }

    fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    fn hook_key_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(Self::Key, Self::Action) + 'static,
    {
        self.raw_mut()
            .set_key_callback(move |_win, key, _scancode, action, _modifiers| {
                callback(key, action)
            });
    }

    fn hook_mouse_position_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(f64, f64) + 'static,
    {
        self.raw_mut()
            .set_cursor_pos_callback(move |_win, x, y| callback(x, y));
    }

    fn hook_mouse_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(Self::MouseKey, Self::Action) + 'static,
    {
        self.raw_mut()
            .set_mouse_button_callback(move |_win, btn, action, _modifiers| callback(btn, action));
    }

    fn loader_function(&mut self, s: &str) -> *const os::raw::c_void {
        unsafe {
            self.raw
                .as_mut()
                .unwrap_unchecked()
                .get_proc_address(s)
                .map(|f| std::mem::transmute(f))
                .unwrap_or(std::ptr::null())
        }
    }
}

#[derive(Debug)]
pub struct Initialized;
#[derive(Debug)]
pub struct Constructed;
trait WindowFlag {}
impl WindowFlag for Initialized {}
impl WindowFlag for Constructed {}
