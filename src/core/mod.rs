mod handler;
pub use handler::Handlers;

use crate::traits::*;

macro_rules! info {
    ($($arg:tt)+) => {
        log::info!(target: "Engine::Core", $($arg)+)
    };
}

pub struct Engine<Application>
where
    Application: crate::Application,
{
    inner: Application,
    container:
        Handlers<Application::WindowHandler, Application::Renderer, Application::InputHandler>,
}

impl<Application> Engine<Application>
where
    Application: crate::Application,
{
    pub fn init(inner: Application) -> Self {
        let (mut window, input, renderer) = inner.initialize();
        let mut renderer = match renderer {
            Some(mut v) => {
                v.with_context(|s| window.loader_function(s));
                v
            }
            None => Application::Renderer::new(|s| window.loader_function(s)),
        };

        window.set_version(
            Application::Renderer::major_version(),
            Application::Renderer::minor_version(),
        );

        let (width, height) = window.get_framebuffer_size();
        renderer.set_viewport(width, height);

        info!("Engine initialized");

        Self {
            container: Handlers {
                window,
                renderer,
                input,
            },
            inner,
        }
    }

    pub fn run(&mut self) {
        while !self.container.window.should_close() {
            self.container.input.buffer_clear();
            self.container.window.poll_events();

            for event in self.container.window.flush() {
                match self.container.input.handle(event.into()) {
                    EventCallback::None => {}
                    EventCallback::FramebufferResize(w, h) => {
                        self.container.renderer.set_viewport(w, h);
                    }
                }
            }

            // Window color buffer clear
            self.container.renderer.clear();

            // User update method call
            self.inner.update(&mut self.container);
            // User render method call
            self.inner.render(&self.container.renderer);

            // Window buffer swap
            self.container.window.swap_buffer();
        }
    }
}
