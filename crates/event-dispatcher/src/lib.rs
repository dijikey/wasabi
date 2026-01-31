use std::collections::HashMap;

pub use glfw::Action;
pub use glfw::Key;
pub use glfw::MouseButton;

#[derive(Debug, Default)]
pub struct Dispatcher {
    keys: HashMap<u32, glfw::Action>,
    mouse_position: (f64, f64),
    window_position: (i32, i32),
    mouse: HashMap<u32, glfw::Action>,
}

impl Dispatcher {
    pub fn get_key(&self, key: glfw::Key) -> Option<&Action> {
        self.keys.get(&(key as u32))
    }

    pub fn get_mouse(&self, key: glfw::MouseButton) -> Option<&Action> {
        self.mouse.get(&(key as u32))
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn window_position(&self) -> (i32, i32) {
        self.window_position
    }
}

impl wasabi_traits::InputHandler for Dispatcher {
    type Event = (f64, glfw::WindowEvent);

    fn handle(&mut self, event: Self::Event) -> wasabi_traits::EventCallback {
        match event.1 {
            glfw::WindowEvent::CursorPos(x, y) => self.mouse_position = (x, y),
            glfw::WindowEvent::Pos(x, y) => self.window_position = (x, y),
            glfw::WindowEvent::FramebufferSize(w, h) => {
                return wasabi_traits::EventCallback::FramebufferResize(w, h);
            }
            glfw::WindowEvent::MouseButton(mouse_button, action, _) => {
                self.mouse.insert(mouse_button as _, action);
            }
            glfw::WindowEvent::Key(key, _, action, _) => {
                self.keys.insert(key as _, action);
            }
            _ => {} // glfw::WindowEvent::Pos(_, _) => todo!(),
                    // glfw::WindowEvent::Size(_, _) => todo!(),
                    // glfw::WindowEvent::Close => todo!(),
                    // glfw::WindowEvent::Refresh => todo!(),
                    // glfw::WindowEvent::Focus(_) => todo!(),
                    // glfw::WindowEvent::Iconify(_) => todo!(),
                    // glfw::WindowEvent::MouseButton(mouse_button, action, modifiers) => todo!(),
                    // glfw::WindowEvent::CursorEnter(_) => todo!(),
                    // glfw::WindowEvent::Scroll(_, _) => todo!(),
                    // glfw::WindowEvent::Key(key, _, action, modifiers) => todo!(),
                    // glfw::WindowEvent::Char(_) => todo!(),
                    // glfw::WindowEvent::CharModifiers(_, modifiers) => todo!(),
                    // glfw::WindowEvent::FileDrop(path_bufs) => todo!(),
                    // glfw::WindowEvent::Maximize(_) => todo!(),
                    // glfw::WindowEvent::ContentScale(_, _) => todo!(),
        };

        wasabi_traits::EventCallback::None
    }

    fn buffer_clear(&mut self) {
        self.keys.clear();
    }
}
