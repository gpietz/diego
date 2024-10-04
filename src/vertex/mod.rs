mod textured_vertex;

pub trait VertexEq<T> {
    fn is_similar(&self, other: &T, tolerance: f32) -> bool;  
}
