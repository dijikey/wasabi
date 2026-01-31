use std::os;

use glfw::{Context, GlfwReceiver, PWindow};
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
    pub receiver: Option<GlfwReceiver<(f64, glfw::WindowEvent)>>,
    pub raw: Option<glfw::PWindow>,
    marker: std::marker::PhantomData<Flag>,
}

#[allow(private_bounds)]
impl<F: WindowFlag> Window<F> {
    pub fn window_hint(&mut self, hint: WindowHint) {
        self.glfw.window_hint(hint);
    }
}

impl Window<Constructed> {
    pub fn new() -> Result<Self, WindowError> {
        use glfw::fail_on_errors;
        let glfw = glfw::init(fail_on_errors!())?;

        Ok(Window {
            glfw,
            receiver: None,
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

        let (mut window, receiver) = match self.glfw.create_window(w, h, title, mode) {
            Some(v) => v,
            None => return None,
        };

        window.set_all_polling(true);

        Some(Window {
            glfw: self.glfw,
            receiver: Some(receiver),
            raw: Some(window),
            marker: std::marker::PhantomData,
        })
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
    type Event = (f64, glfw::WindowEvent);

    fn should_close(&self) -> bool {
        self.raw_ref().should_close()
    }

    fn flush(&self) -> impl Iterator<Item = Self::Event> {
        glfw::flush_messages(unsafe { self.receiver.as_ref().unwrap_unchecked() })
    }

    fn swap_buffer(&mut self) {
        self.raw_mut().swap_buffers();
    }

    fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    fn set_version(&mut self, major: u32, minor: u32) {
        self.window_hint(WindowHint::ContextVersion(major, minor));
        #[cfg(target_os = "macos")]
        self.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        self.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
    }

    fn get_framebuffer_size(&self) -> (i32, i32) {
        self.raw_ref().get_framebuffer_size()
    }

    fn loader_function(&mut self, s: &str) -> *const os::raw::c_void {
        unsafe {
            self.raw_mut()
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
