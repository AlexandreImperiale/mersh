/// Associated geometrical tolerance.
pub const GEOMETRICAL_TOLERANCE: f64 = 1e-12;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining 2d coordinates.
#[derive(Clone, Default)]
pub struct Coord2d {
    /// First coordinate.
    pub x: f64,
    /// Second coordinate.
    pub y: f64,
}

/// Structure for defining 2d points.
#[derive(Clone, Default)]
pub struct Pnt2d {
    /// Coordinates associated to the point.
    pub coords: Coord2d,
}

/// Structure for defining 2d vectors.
#[derive(Clone, Default)]
pub struct Vec2d {
    /// Coordinates associated to the vector.
    pub coords: Coord2d,
}

/// Structure for defining 2d directions (i.e. unit vectors).
#[derive(Clone, Default)]
pub struct Dir2d {
    /// Coordinates associated to the direction.
    pub coords: Coord2d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining 2d coordinates.
#[derive(Clone, Default)]
pub struct Coord3d {
    /// First coordinate.
    pub x: f64,
    /// Second coordinate.
    pub y: f64,
    /// Third coordinate.
    pub z: f64,
}

/// Structure for defining 3d points.
#[derive(Clone, Default)]
pub struct Pnt3d {
    /// Coordinates associated to the point.
    pub coords: Coord3d,
}

/// Structure for defining 3d vectors.
#[derive(Clone, Default)]
pub struct Vec3d {
    /// Coordinates associated to the vector.
    pub coords: Coord3d,
}

/// Structure for defining 3d directions (i.e. unit vectors).
#[derive(Clone, Default)]
pub struct Dir3d {
    /// Coordinates associated to the direction.
    pub coords: Coord3d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Coord2d {
    /// Amplifying coordinates by a scalar coefficient.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord2d{x: 1.0, y: 1.0};
    /// c.amplify(3.0);
    /// assert!((c.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Self
    {
        self.x *= a; self.y *= a; self
    }

    /// Adding potentially amplified coordinate to coordinate,
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord2d{x: 1.0, y: 1.0};
    /// c.add(1.0, &Coord2d{x: 10.0, y: 10.0});
    /// assert!((c.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add(&mut self, a: f64, c: &Coord2d) -> &mut Self
    {
        self.x += a * c.x; self.y += a * c.y; self
    }

    /// Creating 2d coordinate using linear combination of two coordinates.
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `c0` - First coordinate in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c1` - Second coordinate in combination.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.09;
    /// let c0 = Coord2d { x: 1.0, y: 3.4 };
    /// let b = -13.6;
    /// let c1 = Coord2d { x: 10.1, y: -6.4 };
    ///
    /// let c2 = Coord2d::mlt_add(a, &c0, b, &c1);
    ///
    /// assert!((c2.x - a * c0.x - b * c1.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c2.y - a * c0.y - b * c1.y).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 1.0, y: 3.4 };
    /// let c1 = Coord2d::mlt_add(1.0, &c0, -1.0, &c0);
    ///
    /// assert!(c1.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(c1.y.abs() < GEOMETRICAL_TOLERANCE);
    ///```
    pub fn mlt_add(a: f64, c0: &Coord2d, b: f64, c1: &Coord2d) -> Self
    {
        Coord2d { x: a * c0.x + b * c1.x, y: a * c0.y + b * c1.y }
    }

    /// Comparing a coordinate with another one using a fixed epsilon. The comparison is done by
    /// computing the square norme of the difference between the two coordinate.
    ///
    /// * `c` - Coordinate to compare with.
    /// * `eps` - Thershold used for fixed-epsilon floating point comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 1.0, y: 3.4 };
    /// let c1 = Coord2d { x: 6.0, y: -13.0 };
    /// let c2 = c0.clone();
    ///
    /// assert!(!c0.equals(&c1, GEOMETRICAL_TOLERANCE));
    /// assert!(c0.equals(&c2, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn equals(&self, c: &Coord2d, eps: f64) -> bool
    {
        Coord2d::mlt_add(1.0, &self, -1.0, &c).sq_norm() < eps
    }

    /// Computing square norm of a 2d coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 2.0, y: 2.0 };
    /// assert!((c0.sq_norm() - 8.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn sq_norm(&self) -> f64
    {
        self.x * self.x + self.y * self.y
    }

    /// Computing norm of a 2d coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 2.0, y: 2.0 };
    /// assert!((c0.norm() - c0.sq_norm().sqrt()) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn norm(&self) -> f64
    {
         self.sq_norm().sqrt()
    }
}

impl Pnt2d {
    /// Creating new point from coordinates.
    ///
    /// * `x` - First coordinate.
    /// * `y` - Second coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::new(1.0, 0.0);
    /// assert!(p.coords.equals(&Coord2d { x: 1.0, y: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(x: f64, y: f64) -> Self
    {
        Pnt2d{ coords: Coord2d { x: x, y: y} }
    }

    /// Computing distance to another 2d points.
    ///
    /// * `q` - Input 2d point to compute the distance from.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new(0.0, 1.0);
    ///
    /// assert!((p.distance_to(&q) - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    /// let p = Pnt2d::default();
    /// assert!(p.distance_to(&p) < GEOMETRICAL_TOLERANCE);
    ///```
    pub fn distance_to(&self, q: &Pnt2d) -> f64
    {
        Coord2d::mlt_add(1.0, &self.coords, -1.0, &q.coords).norm()
    }

    /// Creating new point by adding an input vector.
    ///
    /// * `v` - Input vector used to create point.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::new(1.45, 3.0);
    /// let v = Vec2d::new(-0.1, 4.09);
    /// let q = p.add(&v);
    ///
    /// assert!((q.coords.x - p.coords.x - v.coords.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.y - p.coords.y - v.coords.y).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add(&self, v: &Vec2d) -> Self
    {
        Pnt2d { coords: Coord2d::mlt_add(1.0, &self.coords, 1.0, &v.coords) }
    }

    /// Creating a vector pointing to an input point.
    ///
    /// * `p` - The point to point to.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new(1.0, 0.0);
    ///
    /// let v0 = p.to(&q);
    /// let v1 = q.to(&p);
    ///
    /// assert!(v0.coords.equals(&Coord2d { x: 1.0, y: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// assert!(v1.coords.equals(&Coord2d { x: -1.0, y: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn to(&self, p: &Pnt2d) -> Vec2d
    {
        Vec2d { coords: Coord2d::mlt_add(1.0, &p.coords, -1.0, &self.coords) }
    }
}

impl Vec2d {
    /// Creating new vector from coordinates.
    ///
    /// * `x` - First coordinate.
    /// * `y` - Second coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec2d::new(1.0, 0.0);
    /// assert!(v.coords.equals(&Coord2d { x: 1.0, y: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(x: f64, y: f64) -> Self
    {
        Vec2d{ coords: Coord2d { x: x, y: y} }
    }

    /// Amplifying a vector by a scalar coefficient.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut v = Vec2d::new(1.0, 0.0);
    /// let a = 12.3;
    ///
    /// assert!((v.amplify(a).coords.norm() - a).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Self
    {
        self.coords.amplify(a); self
    }

    /// Computing norm of a vector.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec2d::new(1.0, 0.0);
    /// assert!((v.norm() - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn norm(&self) -> f64
    {
        self.coords.norm()
    }
}

impl Dir2d {
    /// Creating new direction a point to another. Direction coordinates are computed by normalizing
    /// the vector linking input points. If the distence between input points are close to zero,
    /// the direction provided by default is Ex = (1, 0).
    ///
    /// * `p` - First point.
    /// * `q` - Second point.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new(3.0, 0.0);
    /// let d = Dir2d::new(&p, &q);
    /// assert!(d.coords.equals(&Coord2d { x: 1.0, y: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let d = Dir2d::new(&p, &p);
    /// assert!(d.coords.equals(&Coord2d { x: 1.0, y: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(p: &Pnt2d, q: &Pnt2d) -> Self
    {
        let mut v = p.to(q);
        let norm = v.norm();
        if norm < GEOMETRICAL_TOLERANCE {
            Dir2d{ coords: Coord2d { x: 1.0, y: 0.0} }
        } else {
            v.amplify(1.0 / norm);
            Dir2d{ coords: v.coords }
        }
    }

    /// Creating orthogonal direction so that the cross product between the directions belongs to
    /// positive half space.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    /// use std::f64;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new(1.0, 1.0);
    /// let d0 = Dir2d::new(&p, &q); // => Direction is (1 / sqrt(2), 1 / sqrt(2))
    /// let d1 = d0.ortho(); // => Direction is (-1 / sqrt(2), 1 / sqrt(2))
    ///
    /// assert!(d1.coords.equals(&Coord2d { x: -(0.5 as f64).sqrt(), y: (0.5 as f64).sqrt()}, GEOMETRICAL_TOLERANCE));
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let ex = Dir2d::new(&p, &p); // => Direction is by default Ex = (1, 0).
    /// let ey = ex.ortho(); // Direction is Ey = (0, 1).
    /// assert!(ey.coords.equals(&Coord2d { x: 0.0, y: 1.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn ortho(&self) -> Self
    {
        Dir2d { coords: Coord2d { x: -self.coords.y, y: self.coords.x } }
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Coord3d {
    /// Amplifying coordinates by a scalar coefficient.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord3d{x: 1.0, y: 2.0, z: 3.0};
    /// c.amplify(3.0);
    /// assert!((c.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 6.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.z - 9.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Self
    {
        self.x *= a; self.y *= a; self.z *= a; self
    }

    /// Adding potentially amplified coordinate to coordinate,
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord3d{x: 1.0, y: 1.0, z: 1.0};
    /// c.add(1.0, &Coord3d{x: 10.0, y: 10.0, z: 10.0});
    /// assert!((c.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.z - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add(&mut self, a: f64, c: &Coord3d) -> &mut Self
    {
        self.x += a * c.x; self.y += a * c.y; self.z += a * c.y; self
    }

    /// Creating 3d coordinate using linear combination of two coordinates.
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `c0` - First coordinate in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c1` - Second coordinate in combination.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.09;
    /// let c0 = Coord3d { x: 1.0, y: 3.4, z: 2.0 };
    /// let b = -13.6;
    /// let c1 = Coord3d { x: 10.1, y: -6.4, z: 1.0 };
    ///
    /// let c2 = Coord3d::mlt_add(a, &c0, b, &c1);
    ///
    /// assert!((c2.x - a * c0.x - b * c1.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c2.y - a * c0.y - b * c1.y).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c2.z - a * c0.z - b * c1.z).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d { x: 1.0, y: 3.4, z: 4.1 };
    /// let c1 = Coord3d::mlt_add(1.0, &c0, -1.0, &c0);
    ///
    /// assert!(c1.x.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(c1.y.abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(c1.z.abs() < GEOMETRICAL_TOLERANCE);
    ///```
    pub fn mlt_add(a: f64, c0: &Coord3d, b: f64, c1: &Coord3d) -> Self
    {
        Coord3d { x: a * c0.x + b * c1.x, y: a * c0.y + b * c1.y, z: a * c0.z + b * c1.z }
    }

    /// Comparing a coordinate with another one using a fixed epsilon. The comparison is done by
    /// computing the square norme of the difference between the two coordinate.
    ///
    /// * `c` - Coordinate to compare with.
    /// * `eps` - Thershold used for fixed-epsilon floating point comparison.
    ///
    /// # Examples
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d { x: 1.0, y: 3.4, z: 2.0 };
    /// let c1 = Coord3d { x: 6.0, y: -13.0, z: -1.98 };
    /// let c2 = c0.clone();
    ///
    /// assert!(!c0.equals(&c1, GEOMETRICAL_TOLERANCE));
    /// assert!(c0.equals(&c2, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn equals(&self, c: &Coord3d, eps: f64) -> bool
    {
        Coord3d::mlt_add(1.0, &self, -1.0, &c).sq_norm() < eps
    }

    /// Computing square norm of a 3d coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d { x: 2.0, y: 2.0, z: 1.0 };
    /// assert!((c0.sq_norm() - 9.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn sq_norm(&self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computing norm of a 3d coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d { x: 2.0, y: 2.0, z: 23.1 };
    /// assert!((c0.norm() - c0.sq_norm().sqrt()) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn norm(&self) -> f64
    {
         self.sq_norm().sqrt()
    }
}

impl Pnt3d {
    /// Creating new point from coordinates.
    ///
    /// * `x` - First coordinate.
    /// * `y` - Second coordinate.
    /// * `z` - Third coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::new(1.0, 0.0, 2.0);
    /// assert!(p.coords.equals(&Coord3d { x: 1.0, y: 0.0, z: 2.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self
    {
        Pnt3d{ coords: Coord3d { x: x, y: y, z: z} }
    }

    /// Computing distance to another 3d points.
    ///
    /// * `q` - Input 3d point to compute the distance from.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new(0.0, 1.0, 0.0);
    ///
    /// assert!((p.distance_to(&q) - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    /// let p = Pnt3d::new(2.4, -1.9, 1.0);
    /// assert!(p.distance_to(&p) < GEOMETRICAL_TOLERANCE);
    ///```
    pub fn distance_to(&self, q: &Pnt3d) -> f64
    {
        Coord3d::mlt_add(1.0, &self.coords, -1.0, &q.coords).norm()
    }

    /// Creating new point by adding an input vector.
    ///
    /// * `v` - Input vector used to create point.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::new(1.45, 3.0, 2.0);
    /// let v = Vec3d::new(-0.1, 4.09, 0.98);
    /// let q = p.add(&v);
    ///
    /// assert!((q.coords.x - p.coords.x - v.coords.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.y - p.coords.y - v.coords.y).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.z - p.coords.z - v.coords.z).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add(&self, v: &Vec3d) -> Self
    {
        Pnt3d { coords: Coord3d::mlt_add(1.0, &self.coords, 1.0, &v.coords) }
    }

    /// Creating a vector pointing to an input point.
    ///
    /// * `p` - The point to point to.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new(1.0, 0.0, 0.0);
    ///
    /// let v0 = p.to(&q);
    /// let v1 = q.to(&p);
    ///
    /// assert!(v0.coords.equals(&Coord3d { x: 1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// assert!(v1.coords.equals(&Coord3d { x: -1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn to(&self, p: &Pnt3d) -> Vec3d
    {
        Vec3d { coords: Coord3d::mlt_add(1.0, &p.coords, -1.0, &self.coords) }
    }
}

impl Vec3d {
    /// Creating new vector from coordinates.
    ///
    /// * `x` - First coordinate.
    /// * `y` - Second coordinate.
    /// * `z` - Third coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec3d::new(1.0, 0.0, 0.0);
    /// assert!(v.coords.equals(&Coord3d{ x: 1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self
    {
        Vec3d{ coords: Coord3d { x: x, y: y, z: z} }
    }

    /// Amplifying a vector by a scalar coefficient.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut v = Vec3d::new(1.0, 0.0, 0.0);
    /// let a = 12.3;
    ///
    /// assert!((v.amplify(a).coords.norm() - a).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Self
    {
        self.coords.amplify(a); self
    }

    /// Computing norm of a vector.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec3d::new(1.0, 0.0, 0.0);
    /// assert!((v.norm() - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn norm(&self) -> f64
    {
        self.coords.norm()
    }
}

impl Dir3d {
    /// Creating new direction a point to another. Direction coordinates are computed by normalizing
    /// the vector linking input points. If the distence between input points are close to zero,
    /// the direction provided by default is Ex = (1, 0, 0).
    ///
    /// * `p` - First point.
    /// * `q` - Second point.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new(0.0, 3.0, 0.0);
    /// let d = Dir3d::new(&p, &q);
    /// assert!(d.coords.equals(&Coord3d { x: 0.0, y: 1.0, z: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::new(1.0, 2.0, 3.0);
    /// let d = Dir3d::new(&p, &p);
    /// assert!(d.coords.equals(&Coord3d { x: 1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(p: &Pnt3d, q: &Pnt3d) -> Self
    {
        let mut v = p.to(q);
        let norm = v.norm();
        if norm < GEOMETRICAL_TOLERANCE {
            Dir3d{ coords: Coord3d { x: 1.0, y: 0.0, z: 0.0 } }
        } else {
            v.amplify(1.0 / norm);
            Dir3d{ coords: v.coords }
        }
    }
}
