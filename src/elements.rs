extern crate std;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Element data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Edge as a mesh element.
#[derive(Serialize, Deserialize)]
pub struct Edge {
    /// Associated index of vertices in mesh.
    pub indexes: [usize; 2],
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
#[derive(Serialize, Deserialize)]
pub struct Tri {
    /// Associated index of vertices in mesh.
    pub indexes: [usize; 3],
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
#[derive(Serialize, Deserialize)]
pub struct Quad {
    /// Associated index of vertices in mesh.
    pub indexes: [usize; 4],
}

/// Tetrahedron as a mesh element.
#[derive(Serialize, Deserialize)]
pub struct Tet {
    /// Associated index of vertices in mesh.
    pub indexes: [usize; 4],
}

/// Hexahedron as a mesh element.
#[derive(Serialize, Deserialize)]
pub struct Hexa {
    /// Associated index of vertices in mesh.
    pub indexes: [usize; 8],
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Enum for naming topological relations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Definition of local ordering and naming of edges in a triangle.
#[derive(Clone, Copy)]
pub enum EdgeInTri {
    /// Edge connecting point P0 to P1
    Edge01,
    /// Edge connecting point P1 to P2
    Edge12,
    /// Edge connecting point P2 to P0
    Edge20
}

/// Definition of local ordering and naming of edges in a quadrangle.
#[derive(Clone, Copy)]
pub enum EdgeInQuad {
    /// Edge connecting point P0 to P1
    Edge01,
    /// Edge connecting point P1 to P2
    Edge12,
    /// Edge connecting point P2 to P3
    Edge23,
    /// Edge connecting point P3 to P0
    Edge30
}

/// Definition of local ordering and naming of triangles in a quadrangle.
#[derive(Clone, Copy)]
pub enum TriInQuad {
    /// Tri connecting points (P0, P1, P3)
    Tri013,
    /// Tri connecting points (P1, P2, P3)
    Tri123
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Edge {
    /// Creating a new edge.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13];
    /// let edge = Edge::new(idx);
    ///
    /// assert!(edge.indexes == idx);
    /// ```
    pub fn new(indexes: [usize; 2]) -> Edge { Edge { indexes } }
}

impl Tri {
    /// Creating a new tri.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 24];
    /// let tri = Tri::new(idx);
    ///
    /// assert!(tri.indexes == idx);
    /// ```
    pub fn new(indexes: [usize; 3]) -> Tri { Tri { indexes } }
}

impl Quad {
    /// Creating a new quad.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 53, 21];
    /// let quad = Quad::new(idx);
    ///
    /// assert!(quad.indexes == idx);
    /// ```
    pub fn new(indexes: [usize; 4]) -> Quad { Quad { indexes } }
}

impl Tet {
    /// Creating a new tetrahedra.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 34, 98];
    /// let tet = Tet::new(idx);
    ///
    /// assert!(tet.indexes == idx);
    /// ```
    pub fn new(indexes: [usize; 4]) -> Tet { Tet { indexes } }
}

impl Hexa {
    /// Creating a new hexa.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    ///
    /// let idx = [0, 13, 34, 98, 35, 69, 90, 43];
    /// let hexa = Hexa::new(idx);
    ///
    /// assert!(hexa.indexes == idx);
    /// ```
    pub fn new(indexes: [usize; 8]) -> Hexa { Hexa { indexes } }
}