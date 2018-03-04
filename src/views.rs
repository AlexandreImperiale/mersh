use super::base::*;

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
///
/// Local numbering order of triangle is :
///
/// ```text
/// P2
///   *
///   |`\
///   |  `\
///   |    `\
///   |      `\
///   |        `\
///   *----------*
/// P0             P1
/// ```
pub struct TriView2d<'a> {
    /// Reference to vertices of the triangle.
    pub points: [&'a Pnt2d; 3]
}

/// Structure for defining a 2d quad view.
///
/// Local numbering order of quadrangles is :
///
/// ```text
/// P2                P3
///    * ---------- *
///    |            |
///    |            |
///    |            |
///    |            |
///    * ---------- *
/// P0                P1
/// ```
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
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.edges.push(Edge::new_untagged([0, 1]));
    ///
    /// let e = mesh.get_edge_view(0);
    /// assert!((e.get_length() - 1.0) < 1e-10);
    /// ```
    pub fn get_length(&self) -> f64
    {
        self.points[0].to(&self.points[1]).coords.norm()
    }
}

impl<'a> TriView2d<'a> {
    /// Computing area of a 2d view of a triangle in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(0);
    /// let area = tri.get_area();
    /// assert!((area - 0.5).abs() < 1e-10);
    /// ```
    pub fn get_area(&self) -> f64
    {
        let u01 = self.points[0].to(&self.points[1]);
        let u02 = self.points[0].to(&self.points[2]);

        0.5 * (u01.coords.x * u02.coords.y - u01.coords.y * u02.coords.x).abs()
    }

    /// Computing barycenter of a triangle in a mesh.
    ///
    /// # Example
    /// ```
    /// use mersh::elements::*;
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    /// mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));
    /// mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    ///
    /// let tri = mesh.get_tri_view(0);
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
///
/// Local numbering order of triangle is :
///
/// ```text
/// P2
///   *
///   |`\
///   |  `\
///   |    `\
///   |      `\
///   |        `\
///   *----------*
/// P0             P1
/// ```
pub struct TriView3d<'a> {
    /// Reference to vertices of the triangle.
    pub points: [&'a Pnt3d; 3]
}

/// Structure for defining a 3d quad view.
///
/// Local numbering order of quadrangles is :
///
/// ```text
/// P2                P3
///    * ---------- *
///    |            |
///    |            |
///    |            |
///    |            |
///    * ---------- *
/// P0                P1
/// ```
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