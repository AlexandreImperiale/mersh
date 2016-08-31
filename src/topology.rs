extern crate std;

use super::connections::*;
use super::elements::*;
use super::mesh::*;
use std::vec::*;

/// Topology of a vertex in a mesh.
#[derive(Clone)]
pub struct VertexTopology {
    /// Incident edges.
    pub incident_edges: Vec<VertexToEdge>,
    /// Incident triangles.
    pub incident_tris: Vec<VertexToTri>,
}

/// Topology information of an edge in a mesh.
#[derive(Clone)]
pub struct EdgeTopology {
    /// Connections with other edges.
    pub edge_connections: Vec<EdgeToEdge>,
    /// Connections with triangles.
    pub tri_connections: Vec<EdgeToTri>,
}

/// Topology information of a triangle in a mesh.
#[derive(Clone)]
pub struct TriTopology {
    /// Connections with other triangles.
    pub tri_connections: Vec<TriToTri>,
}

/// Tpology information associated to a mesh.
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

impl VertexTopology {
    /// Creating a new, empty, vertex topology.
    ///
    /// # Examples
    /// ```
    /// use mersh::topology::*;
    /// let vertex_topo = VertexTopology::new();
    /// ```
    pub fn new() -> VertexTopology
    {
        VertexTopology { incident_edges: Vec::new(), incident_tris: Vec::new() }
    }
}

impl EdgeTopology {
    /// Creating a new, empty, edge topology.
    ///
    /// # Examples
    /// ```
    /// use mersh::topology::*;
    /// let edge_topo = EdgeTopology::new();
    /// ```
    pub fn new() -> EdgeTopology
    {
        EdgeTopology { edge_connections: Vec::new(), tri_connections: Vec::new() }
    }
}

impl TriTopology {
    /// Creating a new, empty, tri topology.
    ///
    /// # Examples
    /// ```
    /// use mersh::topology::*;
    /// let tri_topo = TriTopology::new();
    /// ```
    pub fn new() -> TriTopology
    {
        TriTopology { tri_connections: Vec::new() }
    }
}

impl<'a> Topology<'a> {
    /// Creating new topology from a 2d mesh.
    ///
    /// * `mesh` - Input 2d mesh the topology is based upon.
    ///
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::topology::*;
    ///
    /// let mesh = Mesh2d::new();
    /// let topo = Topology::new(&mesh);
    /// ```
    pub fn new(mesh: &Mesh2d) -> Topology
    {
        Topology { nvertices: mesh.vertices.len(), elements: &mesh.elements, vertices: Vec::new(), edges: Vec::new(), tris: Vec::new() }
    }

    /// Building vertices topology informations.
    /// If the vertices topology info have already been built, the previous results are cleared.
    ///
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::topology::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., 0)
    ///     .add_vertex(1., 0., 0)
    ///     .add_vertex(0., 1., 0)
    ///     .add_edge(0, 1, 0)
    ///     .add_tri(2, 0, 1, 0);
    ///
    /// let mut topo = Topology::new(&mesh);
    /// topo.build_vertices();
    ///
    /// assert!(topo.vertices.len() == 3);
    ///
    /// assert!(topo.vertices[0].incident_edges.len() == 1);
    /// assert!(topo.vertices[0].incident_tris.len() == 1);
    ///
    /// // Edge is connected to first vertex through its first vertex.
    /// assert!(topo.vertices[0].incident_edges[0].edge_index == 0);
    /// assert!(topo.vertices[0].incident_edges[0].connecting_vertex as usize == 0);
    ///
    /// // Tri is connected to first vertex through its second vertex.
    /// assert!(topo.vertices[0].incident_tris[0].tri_index == 0);
    /// assert!(topo.vertices[0].incident_tris[0].connecting_vertex as usize == 1);
    ///
    /// assert!(topo.vertices[1].incident_edges.len() == 1);
    /// assert!(topo.vertices[1].incident_tris.len() == 1);
    ///
    /// // Edge is connected to second vertex through its second vertex.
    /// assert!(topo.vertices[1].incident_edges[0].edge_index == 0);
    /// assert!(topo.vertices[1].incident_edges[0].connecting_vertex as usize == 1);
    ///
    /// // Tri is connected to second vertex through its third vertex.
    /// assert!(topo.vertices[1].incident_tris[0].tri_index == 0);
    /// assert!(topo.vertices[1].incident_tris[0].connecting_vertex as usize == 2);
    ///
    /// assert!(topo.vertices[2].incident_edges.len() == 0);
    /// assert!(topo.vertices[2].incident_tris.len() == 1);
    ///
    /// // Tri is connected to third vertex through its first vertex.
    /// assert!(topo.vertices[2].incident_tris[0].tri_index == 0);
    /// assert!(topo.vertices[2].incident_tris[0].connecting_vertex as usize == 0);
    /// ```
    pub fn build_vertices(&mut self) -> &mut Self
    {
        // Clearing previous topology infos.
        self.vertices.clear();
        // Allocating vertices topology infos.
        self.vertices.resize(self.nvertices, VertexTopology::new());

        // Loop on edges.
        for (i, e) in self.elements.edges.iter().enumerate()
        {
            self.vertices[e.v[0]].incident_edges.push(VertexToEdge::new(i, 0));
            self.vertices[e.v[1]].incident_edges.push(VertexToEdge::new(i, 1));
        }

        // Loop on triangles.
        for (i, t) in self.elements.tris.iter().enumerate()
        {
            self.vertices[t.v[0]].incident_tris.push(VertexToTri::new(i, 0));
            self.vertices[t.v[1]].incident_tris.push(VertexToTri::new(i, 1));
            self.vertices[t.v[2]].incident_tris.push(VertexToTri::new(i, 2));
        }

        // Returning reference to topology instance.
        self
    }

    /// Building edges topology informations.
    /// If the vertices topology info have already been built, the previous results are cleared.
    ///
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::topology::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., 0)
    ///     .add_vertex(1., 0., 0)
    ///     .add_vertex(1., -1., 0)
    ///     .add_vertex(1., 1., 0)
    ///     .add_edge(0, 1, 0)
    ///     .add_edge(1, 2, 0)
    ///     .add_edge(1, 3, 0)
    ///     .add_tri(1, 0, 3, 0);
    ///
    /// let mut topo = Topology::new(&mesh);
    /// topo.build_edges();
    ///
    /// assert!(topo.edges.len() == 3);
    ///
    /// assert!(topo.edges[0].edge_connections.len() == 2);
    /// assert!(topo.edges[0].tri_connections.len() == 1);
    ///
    /// // First vertex of second edge connects to second vertex of first edge.
    /// assert!(topo.edges[0].edge_connections[0].connecting_vertex as usize == 1);
    /// assert!(topo.edges[0].edge_connections[0].neighbour.connecting_vertex as usize == 0);
    /// assert!(topo.edges[0].edge_connections[0].neighbour.edge_index == 1);
    ///
    /// // First vertex of third edge connects to second vertex of first edge.
    /// assert!(topo.edges[0].edge_connections[1].connecting_vertex as usize == 1);
    /// assert!(topo.edges[0].edge_connections[1].neighbour.connecting_vertex as usize == 0);
    /// assert!(topo.edges[0].edge_connections[1].neighbour.edge_index == 2);
    ///
    /// // First edge of first triangle is the (reversed) first edge.
    /// assert!(topo.edges[0].tri_connections[0].tri_index == 0);
    /// assert!(topo.edges[0].tri_connections[0].connecting_edge.label as usize == 0);
    /// assert!(topo.edges[0].tri_connections[0].connecting_edge.is_reversed == true);
    /// ```
    pub fn build_edges(&mut self) -> &mut Self
    {
        // Clearing previous topology infos.
        self.edges.clear();
        // Allocating edges topology infos.
        self.edges.resize(self.elements.edges.len(), EdgeTopology::new());

        // Building incident edge indices.
        for (i0, e0) in self.elements.edges.iter().enumerate() {
            for (i1, e1) in self.elements.edges.iter().enumerate() {
                if i1 != i0 {
                    if e1.v[0] == e0.v[0] { self.edges[i0].edge_connections.push(EdgeToEdge::new(0, VertexToEdge::new(i1, 0))); }
                    if e1.v[1] == e0.v[0] { self.edges[i0].edge_connections.push(EdgeToEdge::new(0, VertexToEdge::new(i1, 1))); }
                    if e1.v[0] == e0.v[1] { self.edges[i0].edge_connections.push(EdgeToEdge::new(1, VertexToEdge::new(i1, 0))); }
                    if e1.v[1] == e0.v[1] { self.edges[i0].edge_connections.push(EdgeToEdge::new(1, VertexToEdge::new(i1, 1))); }
                }
            }
        }

        // Bulding incident triangles indices.
        for (i, e) in self.elements.edges.iter().enumerate() {
            for (j, t) in self.elements.tris.iter().enumerate() {
                // Extracting edge position.
                let result = EdgeToTri::get_edge_position(e, t);
                // Adding triangle to edge neighbour list.
                if result.is_some() {
                    let position : EdgePosition = result.unwrap();
                    self.edges[i].tri_connections.push(EdgeToTri::new(j, position.label as usize, position.is_reversed));
                }
            }
        }

        // Returning reference to topology instance.
        self
    }

    /// Building triangles topology informations.
    ///
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::topology::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., 0)
    ///     .add_vertex(1., 0., 0)
    ///     .add_vertex(1., 1., 0)
    ///     .add_vertex(0., 1., 0)
    ///     .add_tri(0, 1, 3, 0)
    ///     .add_tri(1, 2, 3, 0);
    ///
    /// let mut topo = Topology::new(&mesh);
    /// topo.build_triangles();
    ///
    /// assert!(topo.tris.len() == 2);
    ///
    /// // Second edge of first triangle is neighbouring the third (reversed) edge of second triangle.
    /// assert!(topo.tris[0].tri_connections[0].connecting_edge as usize == 1);
    /// assert!(topo.tris[0].tri_connections[0].neighbour.tri_index == 1);
    /// assert!(topo.tris[0].tri_connections[0].neighbour.connecting_edge.label as usize == 2);
    /// assert!(topo.tris[0].tri_connections[0].neighbour.connecting_edge.is_reversed == true);
    ///
    /// ```
    pub fn build_triangles(&mut self) -> &mut Self
    {
        // Clearing previous topology infos.
        self.tris.clear();
        // Allocating triangles topology infos.
        self.tris.resize(self.elements.tris.len(), TriTopology::new());

        // Building incident triangles indices.
        for (i0, t0) in self.elements.tris.iter().enumerate() {
            for (i1, t1) in self.elements.tris.iter().enumerate() {
                if i0 != i1 {
                    // Testing if edge belongs to the triangle.
                    let result = TriToTri::get_common_edge(t0, t1);
                    if result.is_some() {
                        // Creating edge to triangle connection.
                        let edge_to_tri = EdgeToTri::new(i1, result.unwrap().1.label as usize, result.unwrap().1.is_reversed);
                        self.tris[i0].tri_connections.push(TriToTri::new(result.unwrap().0 as usize, edge_to_tri));
                    }
                }
            }
        }

        // Returning reference to topology instance.
        self
    }
}
