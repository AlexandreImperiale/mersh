extern crate std;

use super::base::*;
use super::elements::*;
use super::views::*;
use std::vec::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////


/// Structure for defining 2d mesh vertices.
pub struct Vertex2d {
    /// Associated point.
    pub point: Pnt2d,
    /// Associated tag.
    pub tags: Vec<usize>,
}

/// Structure defining a 2d mesh.
pub struct Mesh2d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex2d>,
    /// Set of 2d mesh elements.
    pub elements: Elements2d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Vertex2d {
    /// Creating a new vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `tags` - vector of integers representing specific tags of the vertex.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let v = Vertex2d::new(0., 0., vec![0]);
    /// assert!(v.point.coords.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.point.coords.y.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.tags[0] == 0);
    /// ```
    pub fn new(x: f64, y: f64, tags: Vec<usize>) -> Vertex2d
    {
        Vertex2d { point: Pnt2d::new(x, y), tags }
    }
}

impl Mesh2d {
    /// Creating a new, empty mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mesh = Mesh2d::new();
    ///
    /// assert!(mesh.vertices.len() == 0);
    /// assert!(mesh.elements.line_elements.edges.len() == 0);
    /// assert!(mesh.elements.tris.len() == 0);
    /// ```
    pub fn new() -> Mesh2d
    {
        Mesh2d { vertices: Vec::new(), elements: Elements2d::new() }
    }

    /// Adding a vertex in a mesh.
    ///
    /// * `x` - First coordinate of the vertex to be added to the mesh.
    /// * `y` - Second coordinate of the vertex to be added to the mesh.
    /// * `tags` - Associated tags.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// let x = 0.0;
    /// let y = 1.0;
    /// mesh.add_vertex(x, y, vec![0]);
    ///
    /// assert!(mesh.vertices.len() == 1);
    /// assert!((mesh.vertices[0].point.coords.x - x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((mesh.vertices[0].point.coords.y - y).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(mesh.vertices[0].tags[0] == 0);
    /// ```
    pub fn add_vertex(&mut self, x: f64, y: f64, tags: Vec<usize>) -> &mut Self
    {
        self.vertices.push(Vertex2d::new(x, y, tags));
        self
    }

    /// Adding an edge in a mesh.
    ///
    /// * `v0` - Index of first vertex defining the edge.
    /// * `v1` - Index of second vertex defining the edge.
    /// * `tags` - Associated tags.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_edge(0, 1, vec![0]);
    ///
    /// assert!(mesh.elements.line_elements.edges.len() == 1);
    /// assert!(mesh.elements.line_elements.edges[0].v[0] == 0);
    /// assert!(mesh.elements.line_elements.edges[0].v[1] == 1);
    /// assert!(mesh.elements.line_elements.edges[0].tags[0] == 0);
    /// ```
    pub fn add_edge(&mut self, v0: usize, v1: usize, tags: Vec<usize>) -> &mut Self
    {
        self.elements.line_elements.edges.push(Edge { v: [v0, v1], tags });
        self
    }

    /// Adding a triangle in a mesh.
    ///
    /// * `v0` - Index of first vertex defining the triangle.
    /// * `v1` - Index of second vertex defining the triangle.
    /// * `v2` - Index of third vertex defining the triangle.
    /// * `tags` - Associated tags.
    ///
    /// # Example
    ///
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_tri(0, 1, 2, vec![0]);
    ///
    /// assert!(mesh.elements.tris.len() == 1);
    /// assert!(mesh.elements.tris[0].v[0] == 0);
    /// assert!(mesh.elements.tris[0].v[1] == 1);
    /// assert!(mesh.elements.tris[0].v[2] == 2);
    /// assert!(mesh.elements.tris[0].tags[0] == 0);
    /// ```
    pub fn add_tri(&mut self, v0: usize, v1: usize, v2: usize, tags: Vec<usize>) -> &mut Self
    {
        self.elements.tris.push(Tri {v: [v0, v1, v2], tags } );
        self
    }

    /// Adding a quadrangle in a mesh.
    ///
    /// * `v0` - Index of first vertex defining the quadrangle.
    /// * `v1` - Index of second vertex defining the quadrangle.
    /// * `v2` - Index of third vertex defining the quadrangle.
    /// * `v3` - Index of forth vertex defining the quadrangle.
    /// * `tags` - Associated tags.
    ///
    /// # Example
    ///
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_vertex(1., 1., vec![0])
    ///     .add_quad(0, 1, 2, 3, vec![0]);
    ///
    /// assert!(mesh.elements.quads.len() == 1);
    /// assert!(mesh.elements.quads[0].v[0] == 0);
    /// assert!(mesh.elements.quads[0].v[1] == 1);
    /// assert!(mesh.elements.quads[0].v[2] == 2);
    /// assert!(mesh.elements.quads[0].v[3] == 3);
    /// assert!(mesh.elements.quads[0].tags[0] == 0);
    /// ```
    pub fn add_quad(&mut self, v0: usize, v1: usize, v2: usize, v3: usize, tags: Vec<usize>) -> &mut Self
    {
        self.elements.quads.push(Quad {v: [v0, v1, v2, v3], tags } );
        self
    }

    /// Creating a view to an edge in a mesh from the input edge itself.
    ///
    /// * `edge` - Edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_edge(0, 1, vec![0]);
    ///
    /// let e = mesh.make_edge_view(&mesh.elements.line_elements.edges[0]);
    /// assert!((e.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_edge_view(&self, edge: &Edge) -> EdgeView2d
    {
         EdgeView2d { p0: &self.vertices[edge.v[0]].point, p1: &self.vertices[edge.v[1]].point }
    }

    /// Extracting a view to an edge in a mesh from an edge index.
    ///
    /// * `i` - Index of the edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_edge(0, 1, vec![0]);
    ///
    /// let e = mesh.get_edge_view(0);
    /// assert!((e.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_edge_view(&self, i: usize) -> EdgeView2d
    {
        self.make_edge_view(&self.elements.line_elements.edges[i])
    }

    /// Making a view to a triangle in a mesh from the element itself.
    ///
    /// * `tri` - Triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_tri(0, 1, 2, vec![0]);
    ///
    /// let tri = mesh.make_tri_view(&mesh.elements.tris[0]);
    /// assert!((tri.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_tri_view(&self, tri: &Tri) -> TriView2d
    {
        TriView2d {
            p0: &self.vertices[tri.v[0]].point,
            p1: &self.vertices[tri.v[1]].point,
            p2: &self.vertices[tri.v[2]].point }
    }

    /// Extracting a view to a triangle in a mesh from its index.
    ///
    /// * `i` - Index of the triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_tri(0, 1, 2, vec![0]);
    ///
    /// let tri = mesh.get_tri_view(0);
    /// assert!((tri.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_tri_view(&self, i: usize) -> TriView2d
    {
       self.make_tri_view(&self.elements.tris[i])
    }

    /// Making a view to a quadrangle in a mesh from the element itself.
    ///
    /// * `quad` - Quadrangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_vertex(1., 1., vec![0])
    ///     .add_quad(0, 1, 2, 3,  vec![0]);
    ///
    /// let quad = mesh.make_quad_view(&mesh.elements.quads[0]);
    /// assert!((quad.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_quad_view(&self, quad: &Quad) -> QuadView2d
    {
        QuadView2d {
            p0: &self.vertices[quad.v[0]].point,
            p1: &self.vertices[quad.v[1]].point,
            p2: &self.vertices[quad.v[2]].point,
            p3: &self.vertices[quad.v[2]].point}
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining 3d mesh vertices.
pub struct Vertex3d {
    /// Associated point.
    pub point: Pnt3d,
    /// Associated tag.
    pub tags: Vec<usize>,
}

/// Structure defining a 3d mesh.
pub struct Mesh3d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex3d>,
    /// Set of 3d mesh elements.
    pub elements: Elements3d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Vertex3d {
    /// Creating a new vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `z` - Second coordinate of the vertex.
    /// * `tags` - vector of integers representing specific tags of the vertex.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let v = Vertex2d::new(0., 0., vec![0]);
    /// assert!(v.point.coords.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.point.coords.y.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.tags[0] == 0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64, tags: Vec<usize>) -> Vertex3d
    {
        Vertex3d { point: Pnt3d::new(x, y, z), tags }
    }
}

impl Mesh3d {
    /// Creating a new, empty mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mesh = Mesh3d::new();
    ///
    /// assert!(mesh.vertices.len() == 0);
    ///
    /// assert!(mesh.elements.line_elements.edges.len() == 0);
    ///
    /// assert!(mesh.elements.surface_elements.tris.len() == 0);
    /// assert!(mesh.elements.surface_elements.quads.len() == 0);
    ///
    /// assert!(mesh.elements.tet.len() == 0);
    /// assert!(mesh.elements.hexa.len() == 0);
    /// assert!(mesh.elements.prism.len() == 0);
    /// ```
    pub fn new() -> Mesh3d
    {
        Mesh3d { vertices: Vec::new(), elements: Elements3d::new() }
    }
}