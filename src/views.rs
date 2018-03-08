use super::base::*;
use super::elements::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining a 2d edge view.
pub struct EdgeView2d<'a> {
    /// Reference to vertices of the edge.
    pub points: [&'a Pnt2d; 2]
}

/// Structure for defining a 2d tri view.
pub struct TriView2d<'a> {
    /// Reference to vertices of the triangle.
    pub points: [&'a Pnt2d; 3]
}

/// Structure for defining a 2d quad view.
pub struct QuadView2d<'a> {
    /// Reference to vertices of the quadrangle.
    pub points: [&'a Pnt2d; 4]
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl<'a> EdgeView2d<'a> {
    /// Computing length of a 2d view to an edge in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 0.]));
    /// mesh.edges.push(Edge::new([0, 1]));
    ///
    /// let e = mesh.get_edge_view(&mesh.edges[0]);
    /// assert!((e.get_length() - 1.0) < 1e-10);
    /// ```
    pub fn get_length(&self) -> f64
    {
        self.points[0].to(self.points[1]).coords.norm()
    }
}

impl<'a> TriView2d<'a> {
    /// Accessing view to a local edge in a triangle
    ///
    /// * `edge_name` - Local name of the edge in the triangle.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 0.]));
    /// mesh.vertices.push(Pnt2d::new([0., 1.]));
    /// mesh.triangles.push(Tri::new([0, 1, 2]));
    ///
    /// let e01 = mesh
    ///     .get_tri_view(&mesh.triangles[0])
    ///     .get_edge_view(EdgeInTri::Edge01);
    ///
    /// assert!((e01.points[0].coords.x).abs() < 1e-10);
    /// assert!((e01.points[0].coords.y).abs() < 1e-10);
    /// assert!((e01.points[1].coords.x - 1.).abs() < 1e-10);
    /// assert!((e01.points[1].coords.y).abs() < 1e-10);
    /// ```
    pub fn get_edge_view<'b>(&'b self, edge_name: EdgeInTri) -> EdgeView2d<'a>
    {
         match edge_name {
             EdgeInTri::Edge01 => EdgeView2d{ points:[self.points[0], self.points[1]] },
             EdgeInTri::Edge12 => EdgeView2d{ points:[self.points[1], self.points[2]] },
             EdgeInTri::Edge20 => EdgeView2d{ points:[self.points[2], self.points[0]] },
         }
    }

    /// Computing area of a 2d view of a triangle in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 0.]));
    /// mesh.vertices.push(Pnt2d::new([0., 1.]));
    /// mesh.triangles.push(Tri::new([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(&mesh.triangles[0]);
    /// let area = tri.get_area();
    /// assert!((area - 0.5).abs() < 1e-10);
    /// ```
    pub fn get_area(&self) -> f64
    {
        let u01 = self.points[0].to(self.points[1]);
        let u02 = self.points[0].to(self.points[2]);

        0.5 * (u01.coords.x * u02.coords.y - u01.coords.y * u02.coords.x).abs()
    }

    /// Computing barycenter of a triangle in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 0.]));
    /// mesh.vertices.push(Pnt2d::new([0., 1.]));
    /// mesh.triangles.push(Tri::new([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(&mesh.triangles[0]);
    /// let bary = tri.get_barycenter();
    /// assert!((bary.coords.x - 0.5).abs() < 1e-10);
    /// assert!((bary.coords.y - 0.5).abs() < 1e-10);
    /// ```
    pub fn get_barycenter(&self) -> Pnt2d
    {
        let mut bary = self.points[0].coords.clone();

        bary.add_in(1.0, &self.points[1].coords)
            .add_in(1.0, &self.points[2].coords)
            .amplify_in(0.5);

        Pnt2d{ coords: bary }
    }
}

impl<'a> QuadView2d<'a> {
    /// Accessing view to a local edge in a quadrangle
    ///
    /// * `edge_name` - Local name of the edge in the quadrangle.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 0.]));
    /// mesh.vertices.push(Pnt2d::new([1., 1.]));
    /// mesh.vertices.push(Pnt2d::new([0., 1.]));
    /// mesh.quadrangles.push(Quad::new([0, 1, 2, 3]));
    ///
    /// let e23 = mesh
    ///     .get_quad_view(&mesh.quadrangles[0])
    ///     .get_edge_view(EdgeInQuad::Edge23);
    ///
    /// assert!((e23.points[0].coords.x - 1.).abs() < 1e-10);
    /// assert!((e23.points[0].coords.y - 1.).abs() < 1e-10);
    /// assert!((e23.points[1].coords.x).abs() < 1e-10);
    /// assert!((e23.points[1].coords.y - 1.).abs() < 1e-10);
    /// ```
    pub fn get_edge_view<'b>(&'b self, edge_name: EdgeInQuad) -> EdgeView2d<'a>
    {
         match edge_name {
             EdgeInQuad::Edge01 => EdgeView2d{ points:[self.points[0], self.points[1]] },
             EdgeInQuad::Edge12 => EdgeView2d{ points:[self.points[1], self.points[2]] },
             EdgeInQuad::Edge23 => EdgeView2d{ points:[self.points[2], self.points[3]] },
             EdgeInQuad::Edge30 => EdgeView2d{ points:[self.points[3], self.points[0]] },
         }
    }

    /// Accessing view to a local triangle in a quadrangle
    ///
    /// * `tri_name` - Local name of the triangle in the quadrangle.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::default();
    ///
    /// mesh.vertices.push(Pnt2d::new([0.0, 0.]));
    /// mesh.vertices.push(Pnt2d::new([2.5, 0.]));
    /// mesh.vertices.push(Pnt2d::new([2.5, 1.]));
    /// mesh.vertices.push(Pnt2d::new([0.0, 1.]));
    /// mesh.quadrangles.push(Quad::new([0, 1, 2, 3]));
    ///
    /// let t013 = mesh.get_quad_view(&mesh.quadrangles[0]).get_tri_view(TriInQuad::Tri013);
    /// let t123 = mesh.get_quad_view(&mesh.quadrangles[0]).get_tri_view(TriInQuad::Tri123);
    /// let area = t013.get_area() + t123.get_area();
    ///
    /// assert!((area - 2.5).abs() < 1e-10);
    /// ```
    pub fn get_tri_view<'b>(&'b self, tri_name: TriInQuad) -> TriView2d<'a>
    {
        match tri_name {
            TriInQuad::Tri013 => TriView2d{ points: [self.points[0], self.points[1], self.points[3]] },
            TriInQuad::Tri123 => TriView2d{ points: [self.points[1], self.points[2], self.points[3]] },
        }
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining a 3d edge view.
pub struct EdgeView3d<'a> {
    /// Reference to vertices of the edge.
    pub points: [&'a Pnt3d; 2]
}

/// Structure for defining a 3d tri view.
pub struct TriView3d<'a> {
    /// Reference to vertices of the triangle.
    pub points: [&'a Pnt3d; 3]
}

/// Structure for defining a 3d quad view.
pub struct QuadView3d<'a> {
    /// Reference to vertices of the quadrangle.
    pub points: [&'a Pnt3d; 4]
}

/// Structure for defining a tetrahedron view.
pub struct TetView3d<'a> {
    /// Reference to vertices of the tetrahedron.
    pub points: [&'a Pnt3d; 4]
}

/// Structure for defining a hexahedron view.
pub struct HexaView3d<'a> {
    /// Reference to vertices of the hexahedron.
    pub points: [&'a Pnt3d; 8]
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl<'a> EdgeView3d<'a> {
    /// Computing length of a 3d view to an edge in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
    /// mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
    /// mesh.edges.push(Edge::new([0, 1]));
    ///
    /// let e = mesh.get_edge_view(&mesh.edges[0]);
    /// assert!((e.get_length() - 1.0) < 1e-10);
    /// ```
    pub fn get_length(&self) -> f64
    {
        self.points[0].to(self.points[1]).coords.norm()
    }
}

impl<'a> TriView3d<'a> {
    /// Computing normal vector associated to a triangle.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Pnt3d::new([0.0, 0.0, 0.0]));
    /// mesh.vertices.push(Pnt3d::new([0.0, 1.0, 0.0]));
    /// mesh.vertices.push(Pnt3d::new([1.0, 0.0, 0.0]));
    /// mesh.triangles.push(Tri::new([0, 1, 2]));
    /// mesh.triangles.push(Tri::new([0, 2, 1]));
    ///
    /// let n = mesh.get_tri_view(&mesh.triangles[0]).get_normal();
    /// let m = mesh.get_tri_view(&mesh.triangles[1]).get_normal();
    ///
    /// assert!(n.coords.equals(&Coord3d{ x: 0.0, y: 0.0, z:-1.0 }, GEOMETRICAL_TOLERANCE));
    /// assert!(m.coords.equals(&Coord3d{ x: 0.0, y: 0.0, z: 1.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn get_normal(&self) -> Dir3d
    {
        let u : Vec3d = self.points[0].to(self.points[1]);
        let v : Vec3d = self.points[0].to(self.points[2]);
        u.cross_out(&v).normalize_out()
    }

    /// Accessing view to a local edge in a triangle
    ///
    /// * `edge_name` - Local name of the edge in the triangle.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh3d::default();
    ///
    /// mesh.vertices.push(Pnt3d::new([0.0, 0.0, 0.0]));
    /// mesh.vertices.push(Pnt3d::new([0.0, 1.0, 0.0]));
    /// mesh.vertices.push(Pnt3d::new([2.0, 0.0, 0.0]));
    /// mesh.triangles.push(Tri::new([0, 1, 2]));
    ///
    /// let e = mesh.get_tri_view(&mesh.triangles[0]).get_edge_view(EdgeInTri::Edge20);
    ///
    /// assert!((e.get_length() - 2.0).abs() < 1e-10);
    /// ```
    pub fn get_edge_view<'b>(&'b self, edge_name: EdgeInTri) -> EdgeView3d<'a>
    {
        match edge_name {
            EdgeInTri::Edge01 => EdgeView3d{ points: [self.points[0], self.points[1]] },
            EdgeInTri::Edge12 => EdgeView3d{ points: [self.points[1], self.points[2]] },
            EdgeInTri::Edge20 => EdgeView3d{ points: [self.points[2], self.points[0]] },
        }
    }
}