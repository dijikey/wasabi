#![feature(associated_type_defaults)]
mod core;

pub use core::*;

pub mod traits {
    pub use wasabi_traits::*;
}

#[cfg(test)]
mod test;

pub trait Application {
    type InputHandler: traits::InputHandler;
    type WindowHandler: traits::WindowContext<Event: Into<<Self::InputHandler as traits::InputHandler>::Event>>;
    type Renderer: traits::Renderer;

    fn render(&self, renderer: &Self::Renderer);

    fn update(
        &mut self,
        engine: &mut Handlers<Self::WindowHandler, Self::Renderer, Self::InputHandler>,
    );

    fn initialize(
        &self,
    ) -> (
        Self::WindowHandler,
        Self::InputHandler,
        Option<Self::Renderer>,
    );
}
