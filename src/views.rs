use super::base::*;

/// Structure for defining a 2d edge view.
pub struct EdgeView2d<'a> {
    /// Reference to first point in the edge.
    pub p0: &'a Pnt2d,
    /// Reference to second point in the edge.
    pub p1: &'a Pnt2d,
}

/// Structure for definng a 2d tri view.
pub struct TriView2d<'a> {
    /// Reference to first vertex of the triangle.
    pub p0: &'a Pnt2d,
    /// Reference to second vertex of the triangle.
    pub p1: &'a Pnt2d,
    /// Reference to third vertex of the triangle.
    pub p2: &'a Pnt2d,
}

impl<'a> EdgeView2d<'a> {
    /// Computing length of a 2d view to an edge in a mesh.
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_edge(0, 1, vec![0]);
    ///
    /// let e = mesh.view_edge(0);
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
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    ///
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_tri(0, 1, 2, vec![0]);
    ///
    /// let tri = mesh.view_tri(0);
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
    /// # Examples
    /// ```
    /// use mersh::mesh::*;
    ///
    /// let mut mesh = Mesh2d::new();
    /// mesh.add_vertex(0., 0., vec![0])
    ///     .add_vertex(1., 0., vec![0])
    ///     .add_vertex(0., 1., vec![0])
    ///     .add_tri(0, 1, 2, vec![0]);
    ///
    /// let tri = mesh.view_tri(0);
    /// let bary = tri.get_barycenter();
    /// assert!((bary.coords.x - 0.5).abs() < 1e-10);
    /// assert!((bary.coords.y - 0.5).abs() < 1e-10);
    /// ```
    pub fn get_barycenter(&self) -> Pnt2d
    {
        let mut bary = self.p0.coords.clone();
        bary.add_in(1.0, &self.p1.coords).add_in(1.0, &self.p2.coords).amplify_in(0.5);

        Pnt2d{ coords: bary }
    }
}
