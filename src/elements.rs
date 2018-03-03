extern crate std;

use std::vec::*;

/// Edge as a mesh element.
pub struct Edge {
    /// Associated index of vertices in mesh.
    pub v: [usize; 2],
    /// Associated tags.
    pub tags: Vec<usize>,
}

/// Triangle as a mesh element.
pub struct Tri {
    /// Associated index of vertices in mesh.
    pub v: [usize; 3],
    /// Associated tag.
    pub tags: Vec<usize>
}

/// Quadrangle as a mesh element.
pub struct Quad {
    /// Associated index of vertices in mesh.
    pub v: [usize; 4],
    /// Associated tag.
    pub tags: Vec<usize>
}

/// Tetrahedron as a mesh element.
pub struct Tet {
    /// Associated index of vertices in mesh.
    pub v: [usize; 4],
    /// Associated tag.
    pub tags: Vec<usize>
}

/// Hexahedron as a mesh element.
pub struct Hexa {
    /// Associated index of vertices in mesh.
    pub v: [usize; 8],
    /// Associated tag.
    pub tags: Vec<usize>
}

/// Prism as a mesh element.
pub struct Prism {
    /// Associated index of vertices in mesh.
    pub v: [usize; 6],
    /// Associated tag.
    pub tags: Vec<usize>
}

/// Structure for regrouping 1d mesh elements.
pub struct Elements1d {
    /// Set of edges.
    pub edges: Vec<Edge>
}

/// Structure for regrouping 2d mesh elements.
pub struct Elements2d {
    // Set of 1d mesh elements.
    pub line_elements: Elements1d,
    /// Set of triangles.
    pub tris: Vec<Tri>,
    /// Set of quadrangles.
    pub quads: Vec<Quad>,
}

/// Structure for regrouping 3d mesh elements.
pub struct Elements3d {
    /// Set 1d mesh elements.
    pub line_elements: Elements1d,
    /// Set 2d mesh elements.
    pub surface_elements: Elements2d,
    /// Set of tetrahedra.
    pub tet: Vec<Tet>,
    /// Set of hexahedra.
    pub hexa: Vec<Hexa>,
    /// Set of prisms.
    pub prism: Vec<Prism>
}

impl Elements1d {
    // Creating a new set of mesh elements.
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    ///
    /// let elems = Elements1d::new();
    ///
    /// assert!(elems.edges.len() == 0);
    /// ```
    pub fn new() -> Elements1d
    {
        Elements1d { edges: Vec::new() }
    }
}

impl Elements2d {
    // Creating a new set of mesh elements.
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    ///
    /// let elems = Elements2d::new();
    ///
    /// assert!(elems.line_elements.edges.len() == 0);
    /// assert!(elems.tris.len() == 0);
    /// assert!(elems.quads.len() == 0);
    /// ```
    pub fn new() -> Elements2d
    {
        Elements2d { line_elements: Elements1d::new(), tris: Vec::new(), quads: Vec::new() }
    }
}

impl Elements3d {
    // Creating a new set of mesh elements.
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    ///
    /// let elems = Elements3d::new();
    ///
    /// assert!(elems.line_elements.edges.len() == 0);
    ///
    /// assert!(elems.surface_elements.tris.len() == 0);
    /// assert!(elems.surface_elements.quads.len() == 0);
    ///
    /// assert!(elems.tet.len() == 0);
    /// assert!(elems.hexa.len() == 0);
    /// assert!(elems.prism.len() == 0);
    /// ```
    pub fn new() -> Elements3d
    {
        Elements3d {
             line_elements: Elements1d::new(),
             surface_elements: Elements2d::new(),
             tet: Vec::new(), hexa: Vec::new(), prism: Vec::new() }
    }
}
