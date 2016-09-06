/// Computing distance to another 2d points.
pub fn distance_to(&self, q: &Pnt2d) -> f64
{
    Coord2d::mlt_add(1.0, &self.coords, -1.0, &q.coords).norm()
}

/// Creating new point by adding an input vector.
pub fn add(&self, v: &Vec2d) -> Pnt2d
{
    Pnt2d { coords: Coord2d::mlt_add(1.0, &self.coords, 1.0, &v.coords) }
}

/// Creating a vector pointing to an input point.
pub fn to(&self, p: &Pnt2d) -> Vec2d
{
    Vec2d { coords: Coord2d::mlt_add(1.0, &p.coords, -1.0, &self.coords) }
}
