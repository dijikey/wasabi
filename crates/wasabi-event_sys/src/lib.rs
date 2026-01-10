#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use wasabi_traits::input::InputHandler;

type SIZE = u32;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ID(SIZE);
static mut GLOBAL_COUNTER: SIZE = 0;

type KeyCallback = Box<dyn Fn(glfw::Key) + 'static>;
type Callback = Box<dyn Fn() + 'static>;

/// Tag for adding a callback
/// # Use
/// ```
/// use wasabi_event_sys::{EventSystem, Tag};
/// use wasabi_traits::input::InputHandler;
/// let mut system = EventSystem::default();
/// let tag = Tag::KeyPressed(Box::new(|| println!("Key pressed")));
/// let id = system.insert(tag);
/// // You don't need to call the function yourself, it's called inside the engine.
/// system.key_pressed();
/// ```
pub enum Tag {
    KeyPressed(KeyCallback),
    KeyReleased(KeyCallback),
    MousePressed(Callback),
    MouseReleased(Callback),
    MouseMoved(Callback),
    // Tag for window
    WindowResized(Callback),
    WindowClosed(Callback),
    WindowFocus(Callback),
    WindowLostFocus(Callback),
    WindowMoved(Callback),
}

#[derive(Default)]
pub struct EventSystem {
    key_pressed: HashMap<ID, KeyCallback>,
    key_released: HashMap<ID, KeyCallback>,
    mouse_pressed: HashMap<ID, Callback>,
    mouse_released: HashMap<ID, Callback>,
    mouse_moved: HashMap<ID, Callback>,
    window_resized: HashMap<ID, Callback>,
    window_closed: HashMap<ID, Callback>,
    window_focused: HashMap<ID, Callback>,
    window_lost_focus: HashMap<ID, Callback>,
    window_moved: HashMap<ID, Callback>,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem::default()
    }

    pub fn insert(&mut self, func: Tag) -> ID {
        macro_rules! tag_add {
            (tag: $tag:ident, id: $id:ident, $((key: $key:ident, field: $field:ident)),*) => {
                match $tag {
                    $(
                        Tag::$key(function) => {self.$field.insert($id, function);},
                    )*
                }
            };
        }

        let id: ID = unsafe {
            GLOBAL_COUNTER += 1;
            ID(GLOBAL_COUNTER)
        };

        tag_add!(tag: func, id: id,
            (key: KeyPressed, field: key_pressed),
            (key: KeyReleased, field: key_released),
            (key: MousePressed, field: mouse_pressed),
            (key: MouseReleased, field: mouse_released),
            (key: MouseMoved, field: mouse_moved),
            (key: WindowResized, field: window_resized),
            (key: WindowClosed, field: window_closed),
            (key: WindowFocus, field: window_focused),
            (key: WindowLostFocus, field: window_lost_focus),
            (key: WindowMoved, field: window_moved)
        );

        id
    }

    pub fn remove_callback(&mut self, id: ID) {
        self.key_pressed.remove(&id);
        self.key_released.remove(&id);
        self.mouse_pressed.remove(&id);
        self.mouse_released.remove(&id);
        self.mouse_moved.remove(&id);
        self.window_resized.remove(&id);
        self.window_closed.remove(&id);
        self.window_focused.remove(&id);
        self.window_lost_focus.remove(&id);
        self.window_moved.remove(&id);
    }
}

impl Into<SIZE> for ID {
    fn into(self) -> SIZE {
        self.0
    }
}

impl Into<ID> for SIZE {
    fn into(self) -> ID {
        ID(self)
    }
}

impl Debug for EventSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventSystem")
            .field("key_pressed", &self.key_pressed.len())
            .field("key_released", &self.key_released.len())
            .field("mouse_pressed", &self.mouse_pressed.len())
            .field("mouse_released", &self.mouse_released.len())
            .field("mouse_moved", &self.mouse_moved.len())
            .field("window_focused", &self.window_focused.len())
            .field("window_lost_focus", &self.window_lost_focus.len())
            .field("window_moved", &self.window_moved.len())
            .finish()
    }
}

// A temporary solution, then the functions themselves will take their values.
macro_rules! event_impl {
    ($($i:ident),*) => {
        $(
            #[inline]
            fn $i(&mut self){
                self.$i.values().for_each(|f| f());
            }
        )*
    };
}

impl InputHandler for EventSystem {
    type Key = glfw::Key;
    type Action = glfw::Action;

    fn key_callback(&mut self, key: Self::Key, action: Self::Action) {
        match action {
            glfw::Action::Release => &mut self.key_released,
            glfw::Action::Press => &mut self.key_pressed,
            glfw::Action::Repeat => &mut self.key_pressed,
        }
        .values()
        .for_each(|f| f(key));
    }

    event_impl!(
        mouse_pressed,
        mouse_released,
        mouse_moved,
        window_resized,
        window_closed,
        window_focused,
        window_lost_focus,
        window_moved
    );
}
