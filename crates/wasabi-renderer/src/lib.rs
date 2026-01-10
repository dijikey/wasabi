use glow::{Context, HasContext};
use wasabi_traits::Renderer as RendererTrait;

use crate::color::Color;

pub mod color;

#[derive(Debug)]
pub struct Renderer {
    pub context: Context,
    pub clear_color: Color,
}

impl Renderer {
    pub fn set_color(&mut self, color: Color) {
        self.clear_color = color;
    }
}

impl RendererTrait for Renderer {
    fn new<F>(f: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        Self {
            context: unsafe { glow::Context::from_loader_function(f) },
            clear_color: Color::WHITE,
        }
    }

    fn with_context<F>(&mut self, f: F)
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
        unsafe {
            self.context = glow::Context::from_loader_function(f);
        }
    }

    fn clear(&self) {
        unsafe {
            self.context.clear_color(
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            );

            self.context.clear(glow::COLOR_BUFFER_BIT);
        };
    }
}
