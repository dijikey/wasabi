use glfw::Context;
use glfw::{Glfw, InitError};
use wasabi_traits::WindowContext;

pub use glfw::Action;
pub use glfw::Key;
pub use glfw::OpenGlProfileHint;
pub use glfw::WindowHint;
pub use glfw::WindowMode;

#[allow(dead_code)]
pub mod builder;
use crate::builder::WindowBuilder;

#[allow(private_bounds)]
#[derive(Debug)]
pub struct Window<Flag: WindowFlag> {
    pub glfw: Glfw,
    // SAFETY
    pub raw: Option<glfw::PWindow>,
    marker: std::marker::PhantomData<Flag>,
}

#[allow(private_bounds)]
impl<T: WindowFlag> Window<T> {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
}

impl Window<Constructed> {
    pub fn new() -> Result<Self, WindowError> {
        use glfw::fail_on_errors;
        let glfw = glfw::init(fail_on_errors!())?;

        Ok(Self {
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WindowError {
    AlreadyInitialized,
    Internal,
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
    type Key = glfw::Key;
    type Action = glfw::Action;

    fn should_close(&self) -> bool {
        // SAFETY
        unsafe { self.raw.as_ref().unwrap_unchecked().should_close() }
    }

    fn swap_buffer(&mut self) {
        // SAFETY
        unsafe {
            self.raw.as_mut().unwrap_unchecked().swap_buffers();
        }
    }

    fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    fn hook_key_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(Self::Key, Self::Action) + 'static,
    {
        unsafe {
            self.raw.as_mut().unwrap_unchecked().set_key_callback(
                move |_win, key, _scancode, action, _modifiers| callback(key, action),
            );
        }
    }

    // fn hook_key_callback<F: FnMut(Self::Key, Self::Action) + 'static>(&mut self, mut callback: F) {
    //     // SAFETY
    //     unsafe {
    //         self.raw.as_mut().unwrap_unchecked().set_key_callback(
    //             move |_window, key, _scancode, action, _modifiers| callback(key, action),
    //         );
    //     }
    // }

    // fn loader_function<T>(&mut self, s: &str) -> *const T {
    //     unsafe { self.raw.as_mut().unwrap_unchecked().get_proc_address(s) as *const _ }
    // }
}

#[derive(Debug)]
pub struct Initialized;
#[derive(Debug)]
pub struct Constructed;
trait WindowFlag {}
impl WindowFlag for Initialized {}
impl WindowFlag for Constructed {}
