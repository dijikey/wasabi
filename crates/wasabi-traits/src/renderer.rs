pub trait Renderer {
    fn new<F>(f: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void;

    fn with_context<F>(&mut self, _f: F)
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
    }

    fn clear(&self);
}
