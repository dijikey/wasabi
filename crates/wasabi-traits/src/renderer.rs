pub trait Renderer {
    type Target<'a>;

    fn render<'a>(&self, target: Self::Target<'a>);

    // fn iter_render<'a>(&'a self, arr: impl Iterator<Item = &'a Self::Target>) {
    //     arr.for_each(|mesh| self.render(mesh));
    // }

    fn minor_version() -> u32;
    fn major_version() -> u32;
    fn set_viewport(&mut self, width: i32, height: i32);

    fn clear(&self);

    fn new<F>(f: F) -> Self
    where
        F: FnMut(&str) -> *const std::os::raw::c_void;

    fn with_context<F>(&mut self, _f: F)
    where
        F: FnMut(&str) -> *const std::os::raw::c_void,
    {
    }
}
