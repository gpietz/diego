use crate::glx::vertex_array_object::VertexArrayObject;
use crate::glx::vertex_attribute::VertexAttribute;
use crate::glx::vertex_layout::{DynamicVertexLayout, SharedDynamicVertexLayout};
use std::collections::HashMap;
use crate::glx::Bindable;

#[derive(Default)]
pub struct VertexLayoutManager<'a> {
    // We use attributes as key for the layout
    layouts: HashMap<Vec<VertexAttribute>, SharedDynamicVertexLayout>,
    // VAO to attribute mapping
    vao_layouts: HashMap<&'a VertexArrayObject, Vec<VertexAttribute>>,
}

impl<'a> VertexLayoutManager<'a> {
    pub fn activate_layout<I>(&mut self, vao: &'a mut VertexArrayObject, attributes: I)
    where
        I: IntoIterator<Item=VertexAttribute>,
    {
        let layout_attributes: Vec<VertexAttribute> = attributes.into_iter().collect();

        // Check whether the layout already exists (based on the attributes)
        if !self.layouts.contains_key(&layout_attributes) {
            // Create and add a new layout
            let dynamic_layout = DynamicVertexLayout::from_attributes(layout_attributes.clone());
            self.layouts.insert(layout_attributes.clone(), dynamic_layout.into_shared());
        }

        // Get layout via the attributes
        let mut layout = self.layouts.get_mut(&layout_attributes).unwrap();

        // Bind the VAO
        vao.bind();

        // Check whether the VAO already has a layout that matches the attributes
        if !self.vao_layouts.contains_key(vao) || self.vao_layouts.get(vao) != Some(&layout_attributes) {
            // // Assign layout to the VAO
            // layout.borrow_mut().configure(); //TODO Configuration für das VAO (... UPLOAD?)
            // // Save layout attribute assignment for VAO
            // self.vao_layouts.insert(vao, layout_attributes);
        }
    }
}
