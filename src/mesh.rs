extern crate std;

use super::base::*;
use super::elements::*;
use super::views::*;
use std::vec::*;

/// Structure for defining 2d mesh vertices.
pub struct Vertex2d {
    /// Associated point.
    pub point: Pnt2d,
    /// Associated tag.
    pub tags: Vec<usize>,
}

/// Structure for defining 3d mesh vertices.
pub struct Vertex3d {
    /// Associated point.
    pub point: Pnt3d,
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

/// Structure defining a 3d mesh.
pub struct Mesh3d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex3d>,
    /// Set of 3d mesh elements.
    pub elements: Elements3d,
}

impl Vertex2d {
    /// Creating a new vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `tags` - vector of integers representing specific tags of the vertex.
    ///
    /// # Examples
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
        Vertex2d { point: Pnt2d::new(x, y), tags: tags }
    }
}

impl Vertex3d {
    /// Creating a new vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `z` - Second coordinate of the vertex.
    /// * `tags` - vector of integers representing specific tags of the vertex.
    ///
    /// # Examples
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
        Vertex3d { point: Pnt3d::new(x, y, z), tags: tags }
    }
}

impl Mesh2d {
    /// Creating a new, empty mesh.
    ///
    /// # Examples
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
    /// * `tag` - An integer representing a specific tag of the vertex.
    ///
    /// # Examples
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
    /// * `tag` - An integer representing a specific tag of the edge.
    ///
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(0., 0., vec![0])
    ///     .add_edge(0, 1, vec![0]);
    ///
    /// assert!(mesh.elements.line_elements.edges.len() == 1);
    /// assert!(mesh.elements.line_elements.edges[0].v[0] == 0);
    /// assert!(mesh.elements.line_elements.edges[0].v[1] == 1);
    /// assert!(mesh.elements.line_elements.edges[0].tags[0] == 0);
    /// ```
    pub fn add_edge(&mut self, v0: usize, v1: usize, tags: Vec<usize>) -> &mut Self
    {
        self.elements.line_elements.edges.push(Edge { v: [v0, v1], tags: tags });
        self
    }

    /// Adding a triangle in a mesh.
    ///
    /// * `v0` - Index of first vertex defining the triangle.
    /// * `v1` - Index of second vertex defining the triangle.
    /// * `v2` - Index of third vertex defining the triangle.
    /// * `tag` - An integer representing a specific tag of the triangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(0., 0., vec![0])
    ///     .add_vertex(0., 0., vec![0])
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
        self.elements.tris.push(Tri {v: [v0, v1, v2], tags: tags } );
        self
    }

    /// Extracting a view to an edge in a mesh.
    ///
    /// * `i` - Index of the edge in the mesh.
    ///
    /// # Examples
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
    /// let e = mesh.view_edge(0);
    /// assert!((e.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn view_edge(&self, i: usize) -> EdgeView2d
    {
        EdgeView2d {
            p0: &self.vertices[self.elements.line_elements.edges[i].v[0]].point ,
            p1: &self.vertices[self.elements.line_elements.edges[i].v[1]].point }
    }

    /// Extracting a view to a triangle in a mesh.
    ///
    /// * `i` - Index of the triangle in the mesh.
    ///
    /// # Examples
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
    /// let tri = mesh.view_tri(0);
    /// assert!((tri.p1.coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.p1.coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn view_tri(&self, i: usize) -> TriView2d
    {
        TriView2d {
            p0: &self.vertices[self.elements.tris[i].v[0]].point ,
            p1: &self.vertices[self.elements.tris[i].v[1]].point ,
            p2: &self.vertices[self.elements.tris[i].v[2]].point }
    }
}

impl Mesh3d {
    /// Creating a new, empty mesh.
    ///
    /// # Examples
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
