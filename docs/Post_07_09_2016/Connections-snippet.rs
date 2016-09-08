pub enum VertexLabel {
    /// First point label.
    V0,
    /// Second point label.
    V1,
    /// Third point label.
    V2
 }

pub enum EdgeLabel {
    /// First edge label.
    E0,
    /// Second edge label.
    E1,
    /// Third edge label.
    E2
}

pub struct EdgePosition {
    /// Label of the edge within the element.
    pub label: EdgeLabel,
    /// Boolean set to true if the edge is reversed in the element.
    pub is_reversed: bool,
}

pub struct VertexToEdge {
    /// Index of the edge.
    pub edge_index: usize,
    /// Position of the vertex in the edge.
    pub connecting_vertex: VertexLabel,
}

pub struct VertexToTri {
    /// Index of the triangle.
    pub tri_index: usize,
    /// Position of the vertex in the triangle.
    pub connecting_vertex: VertexLabel,
}

pub struct EdgeToEdge {
    /// Position of the connecting vertex in the edge.
    pub connecting_vertex: VertexLabel,
    /// Conection with the neighbouring edge.
    pub neighbour: VertexToEdge,
}

pub struct EdgeToTri {
    /// Index of the triangle.
    pub tri_index: usize,
    /// Position of the edge in the triangle.
    pub connecting_edge: EdgePosition,
}

pub struct TriToTri {
    /// Position of the connecting edge in the triangle.
    pub connecting_edge: EdgeLabel,
    /// Connection with the neighbouring triangle.
    pub neighbour: EdgeToTri,
}

impl EdgeToTri {
    pub fn get_edge_position(e: &Edge, t: &Tri) -> Option<EdgePosition>
    {
        // first face.
        if (e.v[0] == t.v[0]) && (e.v[1] == t.v[1]) { return Some(EdgePosition { label: EdgeLabel::E0, is_reversed: false }); }
        if (e.v[1] == t.v[0]) && (e.v[0] == t.v[1]) { return Some(EdgePosition { label: EdgeLabel::E0, is_reversed: true }); }

        // second face.
        if (e.v[0] == t.v[1]) && (e.v[1] == t.v[2]) { return Some(EdgePosition { label: EdgeLabel::E1, is_reversed: false }); }
        if (e.v[1] == t.v[1]) && (e.v[0] == t.v[2]) { return Some(EdgePosition { label: EdgeLabel::E1, is_reversed: true }); }

        // thrid face.
        if (e.v[0] == t.v[2]) && (e.v[1] == t.v[0]) { return Some(EdgePosition { label: EdgeLabel::E2, is_reversed: false }); }
        if (e.v[1] == t.v[2]) && (e.v[0] == t.v[0]) { return Some(EdgePosition { label: EdgeLabel::E2, is_reversed: true }); }

        None
    }
}

impl TriToTri {
    pub fn get_common_edge(t0: &Tri, t1: &Tri) -> Option<(EdgeLabel, EdgePosition)>
    {
        // first edge of tri.
        {
            let e01 = Edge { v: [t0.v[0], t0.v[1]], tag: t0.tag };
            let result = EdgeToTri::get_edge_position(&e01, &t1);
            if result.is_some() {
                return Some((EdgeLabel::E0, result.unwrap()));
            }
        }

        // second edge of tri.
        {
            let e12 = Edge { v: [t0.v[1], t0.v[2]], tag: t0.tag };
            let result = EdgeToTri::get_edge_position(&e12, &t1);
            if result.is_some() {
                return Some((EdgeLabel::E1, result.unwrap()));
            }
        }

        // third edge of tri.
        {
            let e20 = Edge { v: [t0.v[2], t0.v[0]], tag: t0.tag };
            let result = EdgeToTri::get_edge_position(&e20, &t1);
            if result.is_some() {
                return Some((EdgeLabel::E2, result.unwrap()));
            }
        }

        None
    }
}
