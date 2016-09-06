extern crate std;

use std::vec::*;

/// Edge as a mesh element.
pub struct Edge {
    /// Associated index of vertices in mesh.
    pub v: [usize; 2],
    /// Associated tag.
    pub tag: usize,
}

/// Triangle as a mesh element.
pub struct Tri {
    /// Associated index of vertices in mesh.
    pub v: [usize; 3],
    /// Associated tag.
    pub tag: usize
}

/// Structure for regrouping mesh elements.
pub struct MeshElements {
    /// Set of edges.
    pub edges: Vec<Edge>,
    /// Set of triangles.
    pub tris: Vec<Tri>,
}

impl MeshElements {
    // Creating a new set of mesh elements.
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    ///
    /// let elems = MeshElements::new();
    ///
    /// assert!(elems.edges.len() == 0);
    /// assert!(elems.tris.len() == 0);
    /// ```
    pub fn new() -> MeshElements
    {
        MeshElements { edges: Vec::new(), tris: Vec::new() }
    }
}
