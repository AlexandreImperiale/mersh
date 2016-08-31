use super::base::*;

/// Structure for defining 2D mesh vertices.
pub struct Vertex2d {
    /// Associated point.
    pub point: Pnt2d,
    /// Associated tag.
    pub tag: usize,
}

impl Vertex2d {
    // Creating a new vertex.
    ///
    /// * `x` - First coordinate of the vertex.
    /// * `y` - Second coordinate of the vertex.
    /// * `tag` - An integer representing a specific tag of the vertex.
    ///
    /// # Examples
    /// ```
    /// use mersh::vertices::*;
    ///
    /// let v = Vertex2d::new(0., 0., 0);
    /// assert!(v.point.coords.x.abs() < 1e-10);
    /// assert!(v.point.coords.y.abs() < 1e-10);
    /// assert!(v.tag == 0);
    /// ```
    pub fn new(x: f64, y: f64, tag: usize) -> Vertex2d
    {
        Vertex2d { point: Pnt2d::new(x, y), tag: tag }
    }
}
