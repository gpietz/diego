use std::mem;

macro_rules! determine_vertex_type {
    ([f32; 2]) => {
        println!("Detected: 2-component float (vec2 in GLSL)");
    };
    ([f32; 3]) => {
        println!("Detected: 3-component float (vec3 in GLSL)");
    };
    ([f32; 4]) => {
        println!("Detected: 4-component float (vec4 in GLSL)");
    }; // Du kannst hier weitere Typen hinzufügen, wenn nötig.
}

macro_rules! implement_vertex {
    ($struct_name:ident, $( $field_name:ident ),* ) => {
        impl $struct_name {
            pub fn describe() {
                $(
                    println!("Field: {}", stringify!($field_name));
                    let field_offset = mem::size_of_val(&Self { position: [0.0, 0.0], color: [0.0, 0.0, 0.0] }.$field_name);
                    println!("Offset: {}", field_offset);

                    match stringify!($field_name) {
                        "position" => println!("Detected: 2-component float (vec2 in GLSL)"),
                        "color" => println!("Detected: 3-component float (vec3 in GLSL)"),
                        _ => println!("Unknown field"),
                    }
                )*
            }
        }
    }
}

// Definieren der Vertex-Struktur
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2], // 2D-Position
    color: [f32; 3],    // RGB-Farbe
}

// Verwenden des Makros zur Beschreibung der Struktur
implement_vertex!(Vertex, position, color);

fn main() {
    // Testen der Funktion
    Vertex::describe();
}
