#[cfg(test)]
mod test;

use gethand::Getters;
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use wasabi_traits::event::EventHandler;

type Callback = Box<dyn Fn() + Send + Sync + 'static>;

/// Tag for adding a callback
/// # Use
/// ```
/// use wasabi_event_sys::{EventSystem, Tag};
/// use wasabi_traits::event::EventHandler;
/// let mut system = EventSystem::default();
/// system += Tag::KeyPressed(Box::new(|| println!("Key pressed")));
/// // You don't need to call the function yourself, it's called inside the engine.
/// system.key_pressed();
/// ```
pub enum Tag {
    KeyPressed(Callback),
    KeyReleased(Callback),
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

#[derive(Default, Getters)]
#[only_mut]
pub struct EventSystem {
    key_pressed: Vec<Callback>,
    key_released: Vec<Callback>,
    mouse_pressed: Vec<Callback>,
    mouse_released: Vec<Callback>,
    mouse_moved: Vec<Callback>,
    window_resized: Vec<Callback>,
    window_closed: Vec<Callback>,
    window_focused: Vec<Callback>,
    window_lost_focus: Vec<Callback>,
    window_moved: Vec<Callback>,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem::default()
    }
}

impl AddAssign<Tag> for EventSystem {
    fn add_assign(&mut self, tag: Tag) {
        macro_rules! tag_add {
            (tag: $tag:ident, $((key: $key:ident, field: $field:ident)),*) => {
                match $tag {
                    $(
                    Tag::$key(func) => self.$field.push(func),
                    )*
                }
            };
        }

        tag_add!(tag:tag,
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
                self.$i.iter().for_each(|func| func());
            }
        )*
    };
}

impl EventHandler for EventSystem {
    event_impl!(
        key_pressed,
        key_released,
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
