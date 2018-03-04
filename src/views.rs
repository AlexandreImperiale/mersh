use super::base::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining a 2d edge view.
pub struct EdgeView2d<'a> {
    /// Reference to first point in the edge.
    pub p0: &'a Pnt2d,
    /// Reference to second point in the edge.
    pub p1: &'a Pnt2d,
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
    /// Reference to first vertex of the triangle.
    pub p0: &'a Pnt2d,
    /// Reference to second vertex of the triangle.
    pub p1: &'a Pnt2d,
    /// Reference to third vertex of the triangle.
    pub p2: &'a Pnt2d,
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
    /// Reference to first vertex of the quadrangle.
    pub p0: &'a Pnt2d,
    /// Reference to second vertex of the quadrangle.
    pub p1: &'a Pnt2d,
    /// Reference to third vertex of the quadrangle.
    pub p2: &'a Pnt2d,
    /// Reference to forth vertex of the quadrangle.
    pub p3: &'a Pnt2d,
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
        self.p0.to(&self.p1).coords.norm()
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
        let u01 = self.p0.to(&self.p1);
        let u02 = self.p0.to(&self.p2);

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
        let mut bary = self.p0.coords.clone();

        bary.add_in(1.0, &self.p1.coords)
            .add_in(1.0, &self.p2.coords)
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
    /// Reference to first point in the edge.
    pub p0: &'a Pnt3d,
    /// Reference to second point in the edge.
    pub p1: &'a Pnt3d,
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
    /// Reference to first vertex of the triangle.
    pub p0: &'a Pnt3d,
    /// Reference to second vertex of the triangle.
    pub p1: &'a Pnt3d,
    /// Reference to third vertex of the triangle.
    pub p2: &'a Pnt3d,
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
    /// Reference to first vertex of the quadrangle.
    pub p0: &'a Pnt3d,
    /// Reference to second vertex of the quadrangle.
    pub p1: &'a Pnt3d,
    /// Reference to third vertex of the quadrangle.
    pub p2: &'a Pnt3d,
    /// Reference to forth vertex of the quadrangle.
    pub p3: &'a Pnt3d,
}