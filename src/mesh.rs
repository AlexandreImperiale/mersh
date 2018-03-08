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
}

/// Structure defining a 2d mesh.
#[derive(Default)]
pub struct Mesh2d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex2d>,
    /// Edges of the mesh.
    pub edges: Vec<Edge>,
    /// Triangles of the mesh.
    pub triangles: Vec<Tri>,
    /// Quadrangles of the mesh.
    pub quadrangles: Vec<Quad>,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Vertex2d {
    /// Creating a new untagged vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let v = Vertex2d::new_untagged([0., 0.]);
    /// assert!(v.point.coords.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.point.coords.y.abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn new_untagged(coords: [f64; 2]) -> Vertex2d
    {
        Vertex2d { point: Pnt2d::new(coords[0], coords[1]) }
    }
}

impl Mesh2d {
    /// Creating a view to an edge in a mesh from the input edge itself.
    ///
    /// * `edge` - Edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    ///
    /// mesh.edges.push(Edge::new_untagged([0, 1]));
    ///
    /// let e = mesh.make_edge_view(&mesh.edges[0]);
    /// assert!((e.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_edge_view<'a>(&'a self, edge: &Edge) -> EdgeView2d<'a>
    {
         EdgeView2d { points: [
             &self.vertices[edge.v[0]].point,
             &self.vertices[edge.v[1]].point
         ]}
    }

    /// Extracting a view to an edge in a mesh from its index.
    ///
    /// * `idx` - Index of the edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    ///
    /// mesh.edges.push(Edge::new_untagged([0, 1]));
    ///
    /// let e = mesh.get_edge_view(0);
    /// assert!((e.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_edge_view<'a>(&'a self, idx: usize) -> EdgeView2d<'a>
    {
        self.make_edge_view(&self.edges[idx])
    }

    /// Making a view to a triangle in a mesh from the element itself.
    ///
    /// * `tri` - Triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    ///
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.make_tri_view(&mesh.triangles[0]);
    /// assert!((tri.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_tri_view<'a>(&'a self, tri: &Tri) -> TriView2d<'a>
    {
        TriView2d { points: [
            &self.vertices[tri.v[0]].point,
            &self.vertices[tri.v[1]].point,
            &self.vertices[tri.v[2]].point
        ]}
    }

    /// Extracting a view to a triangle in a mesh from its index.
    ///
    /// * `idx` - Index of the triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    ///
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(0);
    /// assert!((tri.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_tri_view<'a>(&'a self, idx: usize) -> TriView2d<'a>
    {
       self.make_tri_view(&self.triangles[idx])
    }

    /// Making a view to a quadrangle in a mesh from the element itself.
    ///
    /// * `quad` - Quadrangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 1.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    ///
    /// mesh.quadrangles.push(Quad::new_untagged([0, 1, 2, 3]));
    ///
    /// let quad = mesh.make_quad_view(&mesh.quadrangles[0]);
    /// assert!((quad.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[3].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_quad_view<'a>(&'a self, quad: &Quad) -> QuadView2d<'a>
    {
        QuadView2d { points:[
            &self.vertices[quad.v[0]].point,
            &self.vertices[quad.v[1]].point,
            &self.vertices[quad.v[2]].point,
            &self.vertices[quad.v[3]].point
        ]}
    }

    /// Making a view to a quadrangle in a mesh from the index of the quad.
    ///
    /// * `i` - Index of the quadrangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 1.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    ///
    /// mesh.quadrangles.push(Quad::new_untagged([0, 1, 2, 3]));
    ///
    /// let quad = mesh.get_quad_view(0);
    /// assert!((quad.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_quad_view<'a>(&'a self, idx: usize) -> QuadView2d<'a>
    {
        self.make_quad_view(&self.quadrangles[idx])
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
}

/// Structure defining a 3d mesh.
#[derive(Default)]
pub struct Mesh3d {
    /// Associated set of vertices.
    pub vertices: Vec<Vertex3d>,
    /// Edges of the mesh.
    pub edges: Vec<Edge>,
    /// Triangles of the mesh.
    pub triangles: Vec<Tri>,
    /// Quadrangles of the mesh.
    pub quadrangles: Vec<Quad>,
    /// Tetrahedra of the mesh.
    pub tetrahedra: Vec<Tet>,
    /// Hexahedra of the mesh.
    pub hexahedra: Vec<Hexa>,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Vertex3d {
    /// Creating a new untagged vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `z` - Second coordinate of the vertex.
    ///
    /// # Example
    /// ```
    /// use mersh::mesh::*;
    /// use mersh::base::*;
    ///
    /// let v = Vertex3d::new_untagged([0., 0., 0.]);
    /// assert!(v.point.coords.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.point.coords.y.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(v.point.coords.z.abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn new_untagged(coords: [f64; 3]) -> Vertex3d
    {
        Vertex3d { point: Pnt3d::new(coords[0], coords[1], coords[2]) }
    }
}

impl Mesh3d {
    /// Creating a view to an edge in a mesh from the input edge itself.
    ///
    /// * `edge` - Edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    ///
    /// mesh.edges.push(Edge::new_untagged([0, 1]));
    ///
    /// let e = mesh.make_edge_view(&mesh.edges[0]);
    /// assert!((e.points[1].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_edge_view<'a>(&'a self, edge: &Edge) -> EdgeView3d<'a>
    {
         EdgeView3d { points: [
             &self.vertices[edge.v[0]].point,
             &self.vertices[edge.v[1]].point
         ]}
    }

    /// Creating a view to an edge in a mesh from its index.
    ///
    /// * `idx` - Index of the edge in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    ///
    /// mesh.edges.push(Edge::new_untagged([0, 1]));
    ///
    /// let e = mesh.get_edge_view(0);
    /// assert!((e.points[1].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((e.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_edge_view<'a>(&'a self, idx: usize) -> EdgeView3d<'a>
    {
         self.make_edge_view(&self.edges[idx])
    }

    /// Making a view to a triangle in a mesh from the element itself.
    ///
    /// * `tri` - Triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 2.]));
    ///
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.make_tri_view(&mesh.triangles[0]);
    /// assert!((tri.points[2].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[2].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[2].coords.z - 2.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_tri_view<'a>(&'a self, tri: &Tri) -> TriView3d<'a>
    {
        TriView3d { points: [
            &self.vertices[tri.v[0]].point,
            &self.vertices[tri.v[1]].point,
            &self.vertices[tri.v[2]].point
        ]}
    }

    /// Making a view to a triangle in a mesh from its index.
    ///
    /// * `idx` - Index of the triangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 2.]));
    ///
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(0);
    /// assert!((tri.points[2].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[2].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tri.points[2].coords.z - 2.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_tri_view<'a>(&'a self, idx: usize) -> TriView3d<'a>
    {
        self.make_tri_view(&self.triangles[idx])
    }

    /// Making a view to a quadrangle in a mesh from the element itself.
    ///
    /// * `quad` - Quadrangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 1.]));
    ///
    /// mesh.quadrangles.push(Quad::new_untagged([0, 1, 2, 3]));
    ///
    /// let quad = mesh.make_quad_view(&mesh.quadrangles[0]);
    /// assert!((quad.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[3].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[3].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_quad_view<'a>(&'a self, quad: &Quad) -> QuadView3d<'a>
    {
        QuadView3d { points:[
            &self.vertices[quad.v[0]].point,
            &self.vertices[quad.v[1]].point,
            &self.vertices[quad.v[2]].point,
            &self.vertices[quad.v[3]].point
        ]}
    }

    /// Making a view to a quadrangle in a mesh from its index.
    ///
    /// * `idx` - Index of the quadrangle in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 1.]));
    ///
    /// mesh.quadrangles.push(Quad::new_untagged([0, 1, 2, 3]));
    ///
    /// let quad = mesh.get_quad_view(0);
    /// assert!((quad.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[3].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[3].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_quad_view<'a>(&'a self, idx: usize) -> QuadView3d<'a>
    {
        self.make_quad_view(&self.quadrangles[idx])
    }

    /// Making a view to a tetrahedron in a mesh from the element itself.
    ///
    /// * `tet` - Tetrahedron in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    ///
    /// mesh.tetrahedra.push(Tet::new_untagged([0, 1, 2, 3]));
    ///
    /// let tet = mesh.make_tet_view(&mesh.tetrahedra[0]);
    /// assert!((tet.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tet.points[3].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tet.points[3].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_tet_view<'a>(&'a self, tet: &Tet) -> TetView3d<'a>
    {
        TetView3d { points:[
            &self.vertices[tet.v[0]].point,
            &self.vertices[tet.v[1]].point,
            &self.vertices[tet.v[2]].point,
            &self.vertices[tet.v[3]].point
        ]}
    }

    /// Making a view to a tetrahedron in a mesh from its index.
    ///
    /// * `idx` - Index of the tetrahedron in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    ///
    /// mesh.tetrahedra.push(Tet::new_untagged([0, 1, 2, 3]));
    ///
    /// let tet = mesh.get_tet_view(0);
    /// assert!((tet.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tet.points[3].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((tet.points[3].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_tet_view<'a>(&'a self, idx: usize) -> TetView3d<'a>
    {
        self.make_tet_view(&self.tetrahedra[idx])
    }

    /// Making a view to a hexahedron in a mesh from the element itself.
    ///
    /// * `hexa` - Hexahedron in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 1.]));
    ///
    /// mesh.hexahedra.push(Hexa::new_untagged([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// let quad = mesh.make_hexa_view(&mesh.hexahedra[0]);
    /// assert!((quad.points[7].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[7].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[7].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn make_hexa_view<'a>(&'a self, hexa: &Hexa) -> HexaView3d<'a>
    {
        let mut points: [&Pnt3d; 8] = unsafe { std::mem::uninitialized() };
        for i in 0..8 { points[i] = &self.vertices[hexa.v[i]].point; }
        HexaView3d { points }
    }

    /// Making a view to a hexahedron in a mesh from its index.
    ///
    /// * `idx` - Index of the hexahedron in the mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 0.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 0., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([1., 1., 1.]));
    /// mesh.vertices.push(Vertex3d::new_untagged([0., 1., 1.]));
    ///
    /// mesh.hexahedra.push(Hexa::new_untagged([0, 1, 2, 3, 4, 5, 6, 7]));
    ///
    /// let quad = mesh.get_hexa_view(0);
    /// assert!((quad.points[7].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[7].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((quad.points[7].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn get_hexa_view<'a>(&'a self, idx: usize) -> HexaView3d<'a>
    {
        self.make_hexa_view(&self.hexahedra[idx])
    }
}