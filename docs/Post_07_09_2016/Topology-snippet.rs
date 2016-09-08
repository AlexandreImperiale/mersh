pub struct VertexTopology {
    /// Incident edges.
    pub incident_edges: Vec<VertexToEdge>,
    /// Incident triangles.
    pub incident_tris: Vec<VertexToTri>,
}

pub struct EdgeTopology {
    /// Connections with other edges.
    pub edge_connections: Vec<EdgeToEdge>,
    /// Connections with triangles.
    pub tri_connections: Vec<EdgeToTri>,
}

pub struct TriTopology {
    /// Connections with other triangles.
    pub tri_connections: Vec<TriToTri>,
}

pub struct Topology<'a> {
    /// Number of vertices in the topology.
    pub nvertices: usize,
    /// Borrowed mesh elements the topology is based upon.
    pub elements: &'a MeshElements,
    /// Set of vertices topology informations.
    pub vertices: Vec<VertexTopology>,
    /// Set of edges topology informations.
    pub edges: Vec<EdgeTopology>,
    /// Set of triangles topology informations.
    pub tris: Vec<TriTopology>,
}
