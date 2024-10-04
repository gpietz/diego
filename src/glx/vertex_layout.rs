use crate::core::SizeOf;
use crate::glx::vertex_attribute::VertexAttribute;
use std::cell::RefCell;
use std::rc::Rc;

pub trait VertexLayout {
    fn attributes() -> Vec<VertexAttribute>;
    fn layout_size() -> usize {
        Self::attributes().iter().map(|attr| attr.calculate_size()).sum()
    }
}

#[derive(Default, Debug)]
pub struct DynamicVertexLayout {
    attributes: Vec<VertexAttribute>,
}

impl DynamicVertexLayout {
    /// Adds a new `VertexAttribute` to the vertex layout.
    ///
    /// This method appends a new `VertexAttribute` to the list of attributes in the layout.
    /// It allows dynamic modification of the layout by adding attributes after its initial creation.
    ///
    /// # Parameters:
    /// - `attr`: The `VertexAttribute` to be added to the layout.
    ///
    /// # Example:
    /// ```rust-rs
    /// let mut layout = DynamicVertexLayout::new();
    /// let new_attribute = VertexAttribute::new(3, VertexDataType::Float);
    /// layout.add_attribute(new_attribute);
    /// ```
    ///
    /// This method is useful for expanding the vertex layout with additional attributes dynamically.
    pub fn add_attribute(&mut self, attr: VertexAttribute) {
        self.attributes.push(attr);
    }

    /// Calculates and returns the total size of the vertex layout in bytes.
    ///
    /// This method iterates over all the vertex attributes in the layout and sums up their sizes,
    /// which are calculated using the `calculate_size` method of each `VertexAttribute`.
    /// The resulting value represents the total size of the layout in bytes.
    ///
    /// # Returns:
    /// The total size of the vertex layout as a `usize`, which indicates the number of bytes
    /// required for all the attributes in the layout.
    pub fn layout_size(&self) -> usize {
        self.attributes.iter().map(|attr| attr.calculate_size()).sum()
    }

    /// Constructs a new `DynamicVertexLayout` from a list of vertex attributes.
    ///
    /// This method creates a `DynamicVertexLayout` by taking any type that can be 
    /// referenced as a slice of `VertexAttribute` and converting it into the internal
    /// attribute list. The attributes are cloned into the layout.
    ///
    /// # Parameters:
    /// - `attributes`: A type that can be referenced as a slice of `VertexAttribute`, such as a vector or an array.
    ///
    /// # Returns:
    /// A new instance of `DynamicVertexLayout` containing the provided attributes.
    ///
    /// # Example:
    /// ```rust-rs
    /// let attrs = vec![
    ///     VertexAttribute::new(3, VertexDataType::Float),
    ///     VertexAttribute::new(2, VertexDataType::Float),
    /// ];
    /// let layout = DynamicVertexLayout::from_attributes(attrs);
    /// ```
    ///
    /// This function accepts both arrays and vectors for flexibility in input.
    pub fn from_attributes<T: AsRef<[VertexAttribute]>>(attributes: T) -> Self {
        Self {
            attributes: attributes.as_ref().to_vec()
        }
    }

    /// Finalizes the attributes by calculating missing stride and offset values.
    ///
    /// This function performs the following steps:
    ///
    /// 1. **Calculate Stride**: If any attribute has a stride value of 0, it computes the stride
    /// based on the number of components and the data type size for each attribute. It then sets
    /// the stride for attributes where it is missing (set to 0).
    ///
    /// 2. **Calculate Offset**: For each attribute, if the offset is not set, it calculates and
    /// assigns the offset based on the cumulative size of the components and data type size.
    /// The offset starts from 0 and increments as the attributes are processed.
    ///
    /// # Notes:
    /// - The `stride` of an attribute represents the number of bytes between the start of one
    ///   attribute and the start of the next attribute.
    /// - The `offset` represents the byte offset from the beginning of the buffer to the start
    ///   of the attribute data.
    pub fn finalize_attributes(&mut self) {
        // Calculate the stride if it is 0
        if self.attributes.iter().any(|attr| attr.stride == 0) {
            let stride: i32 = self.attributes
                .iter()
                .map(|a| a.components as i32 * a.data_type.size() as i32)
                .sum();
            for attr in &mut self.attributes {
                if attr.stride == 0 {
                    attr.stride = stride;
                }
            }
        }

        // Calculate the offset if it is not set
        let mut current_offset: u32 = 0;
        for attr in &mut self.attributes {
            if attr.offset.is_none() {
                attr.offset = Some(current_offset);
            }
            current_offset += attr.components as u32 * attr.data_type.size() as u32;
        }
    }

    /// Converts the current `DynamicVertexLayout` into a shared `SharedDynamicVertexLayout`.
    ///
    /// This method takes ownership of the `DynamicVertexLayout` and converts it into a
    /// `SharedDynamicVertexLayout` by invoking the `into()` method.
    ///
    /// # Returns:
    /// A `SharedDynamicVertexLayout` instance created from the original layout.
    pub fn into_shared(self) -> SharedDynamicVertexLayout {
        self.into()
    }

    /// Returns a slice of the vertex attributes contained in the layout.
    ///
    /// This method provides a reference to the internal list of `VertexAttribute`s.
    /// The returned slice can be used to inspect the attributes but not modify them.
    ///
    /// # Returns:
    /// A slice of `VertexAttribute`s representing the attributes in the vertex layout.
    pub fn get_attributes(&self) -> &[VertexAttribute] {
        &self.attributes
    }
}

pub type SharedDynamicVertexLayout = Rc<RefCell<DynamicVertexLayout>>;

impl From<DynamicVertexLayout> for SharedDynamicVertexLayout {
    fn from(layout: DynamicVertexLayout) -> Self {
        Rc::new(RefCell::new(layout))
    }
}
