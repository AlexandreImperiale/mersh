pub struct Edge {
    /// Associated index of vertices in mesh.
    pub v: [usize; 2],
    /// Associated tag.
    pub tag: usize,
}

pub struct Tri {
    /// Associated index of vertices in mesh.
    pub v: [usize; 3],
    /// Associated tag.
    pub tag: usize
}

pub struct MeshElements {
    /// Set of vertices index defining edges.
    pub edges: Vec<Edge>,
    /// Set of vertices index defining triangles.
    pub tris: Vec<Tri>,
}
