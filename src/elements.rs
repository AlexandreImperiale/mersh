extern crate std;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Element data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Edge as a mesh element.
pub struct Edge {
    /// Associated index of vertices in mesh.
    pub v: [usize; 2],
}

/// Triangle as a mesh element.
///
/// Local numbering order of triangle is :
///
/// ```text
/// P2
///   *
///   |`\
///   |  `\
///   |    `\
///   |      `\
///   |        `\
///   *----------*
/// P0             P1
/// ```
pub struct Tri {
    /// Associated index of vertices in mesh.
    pub v: [usize; 3],
}

/// Definition of local ordering and naming of edges in a triangle.
pub enum EdgeInTri {
    /// Edge connecting point P0 to P1
    Edge01,
    /// Edge connecting point P1 to P2
    Edge12,
    /// Edge connecting point P2 to P0
    Edge20
}

/// Quadrangle as a mesh element.
///
/// ```text
/// P3                P2
///    * ---------- *
///    |            |
///    |            |
///    |            |
///    |            |
///    * ---------- *
/// P0                P1
/// ```
pub struct Quad {
    /// Associated index of vertices in mesh.
    pub v: [usize; 4],
}

/// Definition of local ordering and naming of edges in a quadrangle.
pub enum EdgeInQuad {
    /// Edge connecting point P0 to P1
    Edge01,
    /// Edge connecting point P1 to P3
    Edge13,
    /// Edge connecting point P3 to P2
    Edge32,
    /// Edge connecting point P2 to P0
    Edge20
}

/// Definition of local ordering and naming of triangles in a quadrangle.
pub enum TriInQuad {
    /// Tri connecting points (P0, P1, P2)
    Tri012,
    /// Tri connecting points (P1, P3, P2)
    Tri132
}

/// Tetrahedron as a mesh element.
pub struct Tet {
    /// Associated index of vertices in mesh.
    pub v: [usize; 4],
}

/// Hexahedron as a mesh element.
pub struct Hexa {
    /// Associated index of vertices in mesh.
    pub v: [usize; 8],
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Edge {
    /// Creating a new untagged edge
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13];
    /// let edge = Edge::new_untagged(idx);
    ///
    /// assert!(edge.v == idx);
    /// ```
    pub fn new_untagged(v: [usize; 2]) -> Edge { Edge { v } }
}

impl Tri {
    /// Creating a new untagged tri
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 24];
    /// let tri = Tri::new_untagged(idx);
    ///
    /// assert!(tri.v == idx);
    /// ```
    pub fn new_untagged(v: [usize; 3]) -> Tri { Tri { v } }
}

impl Quad {
    /// Creating a new untagged quad
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 53, 21];
    /// let quad = Quad::new_untagged(idx);
    ///
    /// assert!(quad.v == idx);
    /// ```
    pub fn new_untagged(v: [usize; 4]) -> Quad { Quad { v } }
}

impl Tet {
    /// Creating a new untagged tetrahedra
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 34, 98];
    /// let tet = Tet::new_untagged(idx);
    ///
    /// assert!(tet.v == idx);
    /// ```
    pub fn new_untagged(v: [usize; 4]) -> Tet { Tet { v } }
}

impl Hexa {
    /// Creating a new untagged hexa
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 34, 98, 35, 69, 90, 43];
    /// let hexa = Hexa::new_untagged(idx);
    ///
    /// assert!(hexa.v == idx);
    /// ```
    pub fn new_untagged(v: [usize; 8]) -> Hexa { Hexa { v } }
}