use crate::traits::*;

pub struct Handlers<WindowHandler, RendererHandler, _InputHandler>
where
    RendererHandler: Renderer,
    _InputHandler: InputHandler,
    WindowHandler: WindowContext,
{
    pub(crate) window: WindowHandler,
    pub(crate) renderer: RendererHandler,
    pub(crate) input: _InputHandler,
}

impl<WindowHandler, RendererHandler, _InputHandler>
    Handlers<WindowHandler, RendererHandler, _InputHandler>
where
    WindowHandler: WindowContext,
    RendererHandler: Renderer,
    _InputHandler: InputHandler,
{
    pub fn window(&mut self) -> &mut WindowHandler {
        &mut self.window
    }
    pub fn renderer(&mut self) -> &mut RendererHandler {
        &mut self.renderer
    }
    pub fn input(&mut self) -> &mut _InputHandler {
        &mut self.input
    }
}
