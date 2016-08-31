use super::elements::*;

/// Labels of a vertex within an element.
#[derive(Copy, Clone)]
pub enum VertexLabel {
    /// First point label.
    V0,
    /// Second point label.
    V1,
    /// Third point label.
    V2
 }

/// Labels of edges within an element.
#[derive(Copy, Clone)]
pub enum EdgeLabel {
    /// First edge label.
    E0,
    /// Second edge label.
    E1,
    /// Third edge label.
    E2
}

/// Position of an edge within en element.
#[derive(Copy, Clone)]
pub struct EdgePosition {
    /// Label of the edge within the element.
    pub label: EdgeLabel,
    /// Boolean set to true if the edge is reversed in the element.
    pub is_reversed: bool,
}

/// Connection of a vertex with an edge.
#[derive(Clone)]
pub struct VertexToEdge {
    /// Index of the edge.
    pub edge_index: usize,
    /// Position of the vertex in the edge.
    pub connecting_vertex: VertexLabel,
}

/// Connection of a vertex with a triangle.
#[derive(Clone)]
pub struct VertexToTri {
    /// Index of the triangle.
    pub tri_index: usize,
    /// Position of the vertex in the triangle.
    pub connecting_vertex: VertexLabel,
}

/// Connection of an edge with an edge.
#[derive(Clone)]
pub struct EdgeToEdge {
    /// Position of the connecting vertex in the edge.
    pub connecting_vertex: VertexLabel,
    /// Conection with the neighbouring edge.
    pub neighbour: VertexToEdge,
}

/// Connection of an edge with a triangle.
#[derive(Clone)]
pub struct EdgeToTri {
    /// Index of the triangle.
    pub tri_index: usize,
    /// Position of the edge in the triangle.
    pub connecting_edge: EdgePosition,
}

/// Connection of a triangle with a triangle.
#[derive(Clone)]
pub struct TriToTri {
    /// Position of the connecting edge in the triangle.
    pub connecting_edge: EdgeLabel,
    /// Connection with the neighbouring triangle.
    pub neighbour: EdgeToTri,
}

impl VertexToEdge {
    /// Creating a new vertex to edge connection.
    ///
    /// * `edge_index` - Index of the edge in the mesh the vertex is connected to.
    /// * `position` - Position of the vertex in the edge.
    ///
    /// # Panics
    /// Panics if either `position` is greater than 1 since it does not represent a vertex in an
    /// edge.
    ///
    /// # Examples
    /// ```
    /// use mersh::connections::*;
    ///
    /// let c = VertexToEdge::new(12, 0);
    ///
    /// assert!(c.edge_index == 12);
    /// match c.connecting_vertex { VertexLabel::V0 => assert!(true), _ => assert!(false)};
    /// assert!(c.connecting_vertex as usize == 0);
    /// ```
    ///
    /// ```rust,should_panic
    /// use mersh::connections::*;
    ///
    /// let c = VertexToEdge::new(12, 2);
    /// ```
    pub fn new(edge_index: usize, position: usize) -> VertexToEdge
    {
        match position {
            0 => VertexToEdge { edge_index: edge_index, connecting_vertex: VertexLabel::V0 },
            1 => VertexToEdge { edge_index: edge_index, connecting_vertex: VertexLabel::V1 },
            _ => panic!("Building vertex to edge connection with irrelevant position of vertex in edge.")
        }
    }
}

impl VertexToTri {
    /// Creating a new vertex to triangle connection.
    ///
    /// * `tri_index` - Index of the triangle in the mesh the vertex is connected to.
    /// * `position` - Position of the vertex in the triangle.
    ///
    /// # Panics
    /// Panics if either `position` is greater than 2 since it does not represent a vertex in a
    /// triangle.
    ///
    /// # Examples
    /// ```
    /// use mersh::connections::*;
    ///
    /// let c = VertexToTri::new(6, 1);
    ///
    /// assert!(c.tri_index == 6);
    /// match c.connecting_vertex { VertexLabel::V1 => assert!(true), _ => assert!(false)};
    /// assert!(c.connecting_vertex as usize == 1);
    /// ```
    ///
    /// ```rust,should_panic
    /// use mersh::connections::*;
    ///
    /// let c = VertexToTri::new(6, 3);
    /// ```
    pub fn new(tri_index: usize, position: usize) -> VertexToTri
    {
        match position {
            0 => VertexToTri { tri_index: tri_index, connecting_vertex: VertexLabel::V0 },
            1 => VertexToTri { tri_index: tri_index, connecting_vertex: VertexLabel::V1 },
            2 => VertexToTri { tri_index: tri_index, connecting_vertex: VertexLabel::V2 },
            _ => panic!("Building vertex to triangle connection with irrelevant position of vertex in triangle.")
        }
    }
}

impl EdgeToEdge {
    /// Creating a new edge to edge connection.
    ///
    /// * `position` - The position of the connecting vertex in the current edge.
    /// * `neighbour` - A vertex to edge connection yielding neighbouring edge information.
    ///
    /// # Panics
    /// Panics if either `position` is greater than 1 since it does not represent a vertex in an
    /// edge.
    ///
    /// # Examples
    /// ```
    /// use mersh::connections::*;
    ///
    /// // The last vertex in edge is connected by the 6th edge at its first vertex.
    /// let c = EdgeToEdge::new(1, VertexToEdge::new(6, 0));
    ///
    /// match c.connecting_vertex { VertexLabel::V1 => assert!(true), _ => assert!(false) };
    /// assert!(c.neighbour.edge_index == 6);
    /// assert!(c.neighbour.connecting_vertex as usize == 0);
    /// ```
    ///
    /// ```rust,should_panic
    /// use mersh::connections::*;
    ///
    /// let c = EdgeToEdge::new(2, VertexToEdge::new(6, 0));
    /// ```
    pub fn new(position: usize, neighbour: VertexToEdge) -> EdgeToEdge
    {
        match position {
            0 => EdgeToEdge { connecting_vertex: VertexLabel::V0, neighbour: neighbour },
            1 => EdgeToEdge { connecting_vertex: VertexLabel::V1, neighbour: neighbour },
            _ => panic!("Building edge to edge connection with irrelevant position of connecting vertex.")
        }
    }

}

impl EdgeToTri {
    /// Creating a new edge to triangle connection.
    ///
    /// * `tri_index` - Index of the triangle in the mesh the edge is connected to.
    /// * `position` - Position of the edge in the triangle.
    /// * `is_reversed` - Boolean set to true if the edge is reversed in the triangle.
    ///
    /// # Panics
    /// Panics if either `position` is greater than 2 since it does not represent an edge in a
    /// triangle.
    ///
    /// # Examples
    /// ```
    /// use mersh::connections::*;
    ///
    /// let c = EdgeToTri::new(6, 1, false);
    ///
    /// assert!(c.tri_index == 6);
    /// match c.connecting_edge.label { EdgeLabel::E1 => assert!(true), _ => assert!(false)};
    /// assert!(c.connecting_edge.label as usize == 1);
    /// assert!(c.connecting_edge.is_reversed == false);
    /// ```
    ///
    /// ```rust,should_panic
    /// use mersh::connections::*;
    ///
    /// let c = EdgeToTri::new(6, 3, false);
    /// ```
    pub fn new(tri_index: usize, position: usize, is_reversed: bool) -> EdgeToTri
    {
        match position {
            0 => EdgeToTri { tri_index: tri_index, connecting_edge: EdgePosition {label: EdgeLabel::E0, is_reversed: is_reversed } },
            1 => EdgeToTri { tri_index: tri_index, connecting_edge: EdgePosition {label: EdgeLabel::E1, is_reversed: is_reversed } },
            2 => EdgeToTri { tri_index: tri_index, connecting_edge: EdgePosition {label: EdgeLabel::E2, is_reversed: is_reversed } },
            _ => panic!("Building edge to vertex connection with irrelevant position of edge in triangle.")
        }
    }

    /// Extracting position of an edge in a triangle. The return value is optional since the edge may not belong
    /// to the triangle.
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    /// use mersh::connections::*;
    ///
    /// let edge = Edge{ v: [1, 0], tag: 0 };
    /// let tri = Tri{ v: [0, 1, 2], tag: 0 };
    /// let edge_position = EdgeToTri::get_edge_position(&edge, &tri);
    ///
    /// assert!(edge_position.is_some());
    /// assert!(edge_position.unwrap().label as usize == 0);
    /// assert!(edge_position.unwrap().is_reversed == true);
    /// ```
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
    /// Creating a new triangle to triangle connection.
    ///
    /// * `position` - The position of the connecting edge in the current triangle.
    /// * `neighbour` - An edge to triangle connection representing the neighbouring triangle.
    ///
    /// # Panics
    /// Panics if either `position` is greater than 2 since it does not represent an edge in a
    /// triangle.
    ///
    /// # Examples
    /// ```
    /// use mersh::connections::*;
    ///
    /// // The last edge in triangle is connected by the 19th triangle at its second edge.
    /// let c = TriToTri::new(2, EdgeToTri::new(19, 1, true));
    ///
    /// match c.connecting_edge { EdgeLabel::E2 => assert!(true), _ => assert!(false) };
    /// assert!(c.neighbour.tri_index == 19);
    /// assert!(c.neighbour.connecting_edge.label as usize == 1);
    /// assert!(c.neighbour.connecting_edge.is_reversed == true);
    /// ```
    ///
    /// ```rust,should_panic
    /// use mersh::connections::*;
    ///
    /// let c = EdgeToEdge::new(2, VertexToEdge::new(6, 0));
    /// ```
    pub fn new(position: usize, neighbour: EdgeToTri) -> TriToTri
    {
        match position {
            0 => TriToTri { connecting_edge: EdgeLabel::E0, neighbour: neighbour },
            1 => TriToTri { connecting_edge: EdgeLabel::E1, neighbour: neighbour },
            2 => TriToTri { connecting_edge: EdgeLabel::E2, neighbour: neighbour },
            _ => panic!("Building triangle to triangle connection with irrelevant position of connecting edge.")
        }
    }

    /// Extract the common edge of two triangles. The result is optional since the two input triangles
    /// may be disjoint.
    ///
    /// * `t0` - The EdgeLabel in the return tuple corresponds to the label of the common edge in
    /// triangle `t0`.
    /// * `t1` - The EdgePosition in the return tuple corresponds to the label of the common edge in
    /// `t1` (potentially reversed w.r.t. the the orientation of the edge in `t0`).
    ///
    /// # Examples
    /// ```
    /// use mersh::elements::*;
    /// use mersh::connections::*;
    ///
    /// let tri0 = Tri{ v: [0, 1, 2], tag: 0 };
    /// let tri1 = Tri{ v: [3, 2, 1], tag: 0 };
    /// let common_edge = TriToTri::get_common_edge(&tri0, &tri1);
    ///
    /// assert!(common_edge.is_some());
    /// assert!(common_edge.unwrap().0 as usize == 1);
    /// assert!(common_edge.unwrap().1.label as usize == 1);
    /// assert!(common_edge.unwrap().1.is_reversed == true);
    /// ```
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
