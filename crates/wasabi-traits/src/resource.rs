/// This trait is optional, designed to simplify the creation of a structure for storing resources.
pub trait ResourceCatalog{
    // Todo:  
    type Mesh;
    type Texture;
    type Sound;
    type Shader;
    type Id: Copy;

    fn get_mesh(&self, id: Self::Id) -> &Self::Mesh;
    fn get_texture(&self, id: Self::Id) -> &Self::Texture;
    fn get_sound(&self, id: Self::Id) -> &Self::Sound;
    fn get_shader(&self, id: Self::Id) -> &Self::Shader;

    fn mesh_search(&self, pattern: &Self::Mesh) -> Self::Id;
    fn texture_search(&self, pattern: &Self::Texture) -> Self::Id;
    fn sound_search(&self, id: Self::Id) -> Self::Id;
    fn shader_search(&self, pattern: &Self::Shader) -> Self::Id;

    fn free_texture(&mut self, id: Self::Id);
    fn free_sound(&mut self, id: Self::Id);
    fn free_shader(&mut self, id: Self::Id);
    fn free_mesh(&mut self, id: Self::Id);
}

trait AsSearcher{
    type Searcher: Searcher;

    fn as_searcher(&self) -> Self::Searcher;
}

trait Searcher{
    fn next_match(&mut self) -> Option<&str>;
}