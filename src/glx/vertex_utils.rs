use crate::vertex::VertexEq;

pub fn dedupe_vertices<T: VertexEq<T> + Clone>(vertices: &[T], tolerance: f32) -> (Vec<T>, Vec<u32>) {
    let mut unique_vertices = Vec::<T>::new();
    let mut indices = Vec::<u32>::new();

    'outer: for vertex in vertices {
        for (index, unique_vertex) in unique_vertices.iter().enumerate() {
            if vertex.is_similar(unique_vertex, tolerance) {
                indices.push(index as u32);
                continue 'outer;
            }
        }
        unique_vertices.push(vertex.clone());
        indices.push(unique_vertices.len() as u32 - 1);
    }

    (unique_vertices, indices)
}
