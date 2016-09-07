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

impl Mesh2d {`
    pub fn view_edge(&self, i: usize) -> EdgeView2d
    {
        EdgeView2d {
            p0: &self.vertices[self.elements.edges[i].v[0]].point ,
            p1: &self.vertices[self.elements.edges[i].v[1]].point }
    }

    pub fn view_tri(&self, i: usize) -> TriView2d
    {
        TriView2d {
            p0: &self.vertices[self.elements.tris[i].v[0]].point ,
            p1: &self.vertices[self.elements.tris[i].v[1]].point ,
            p2: &self.vertices[self.elements.tris[i].v[2]].point }
    }
}

impl<'a> EdgeView2d<'a> {
    pub fn get_length(&self) -> f64
    {
        self.p0.to(&self.p1).norm()
    }
}

impl<'a> TriView2d<'a> {

    pub fn get_area(&self) -> f64
    {
        let u01 = self.p0.to(&self.p1);
        let u02 = self.p0.to(&self.p2);

        0.5 * (u01.coords.x * u02.coords.y - u01.coords.y * u02.coords.x).abs()
    }

    pub fn get_barycenter(&self) -> Pnt2d
    {
        let mut bary = self.p0.coords.clone();
        bary.add(1.0, &self.p1.coords).add(1.0, &self.p2.coords).amplify(0.5);

        Pnt2d{ coords: bary }
    }
}
