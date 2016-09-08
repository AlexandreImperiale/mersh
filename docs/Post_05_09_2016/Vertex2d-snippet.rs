pub struct Vertex2d {
    /// Associated point.
    pub point: Pnt2d,
    /// Associated tag.
    pub tag: usize,
}

pub struct Mesh2d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex2d>,
    /// Set of mesh elements.
    pub elements: MeshElements,
}
