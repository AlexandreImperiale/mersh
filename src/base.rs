extern crate std;

/// Associated geometrical tolerance.
pub const GEOMETRICAL_TOLERANCE: f64 = 1e-12;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining 2d coordinates.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Coord2d {
    /// First coordinate.
    pub x: f64,
    /// Second coordinate.
    pub y: f64,
}

/// Structure for defining 2d points.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Pnt2d {
    /// Coordinates associated to the point.
    pub coords: Coord2d,
}

/// Structure for defining 2d vectors.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Vec2d {
    /// Coordinates associated to the vector.
    pub coords: Coord2d,
}

/// Structure for defining 2d directions (i.e. unit vectors).
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Dir2d {
    /// Coordinates associated to the direction.
    pub coords: Coord2d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 2D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl std::fmt::Display for Coord2d {
    /// Implementing display for 2d coordinates. By default the number of decimal is set to 6.
    ///
    /// * `formatter` - input reference to formatter.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// let coords = Coord2d::new([1.0, 9.0]);
    /// let formatted_coords = format!("{}", coords);
    ///
    /// assert_eq!("(1.000000, 9.000000)", formatted_coords);
    /// ```
    ///
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({:.6}, {:.6})", self.x, self.y)
    }
}

impl Coord2d {
    /// Creating new coordinates.
    ///
    /// * `xy` - associated coordinates values.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let coords = Coord2d::new([1.0, 0.0]);
    /// assert!((coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((coords.y - 0.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn new(xy: [f64; 2]) -> Self
    {
        Coord2d { x: xy[0], y: xy[1] }
    }

    /// Amplifying coordinates by a scalar coefficient. In-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord2d::new([1.0, 1.0]);
    /// c.amplify_in(3.0);
    /// assert!((c.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify_in(&mut self, a: f64) -> &mut Self
    {
        self.x *= a;
        self.y *= a;
        self
    }

    /// Amplifying coordinates by a scalar coefficient. Out-of-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d::new([1.0, 1.0]);
    /// let c1 = c0.amplify_out(3.0);
    /// assert!((c1.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.y - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify_out(&self, a: f64) -> Self
    {
        Coord2d{
            x: a * self.x,
            y: a * self.y }
    }

    /// Adding potentially amplified coordinate to coordinate. In-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord2d::new([1.0, 1.0]);
    /// c.add_in(1.0, &Coord2d{x: 10.0, y: 10.0});
    /// assert!((c.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add_in(&mut self, a: f64, c: &Coord2d) -> &mut Self
    {
        self.x += a * c.x;
        self.y += a * c.y;
        self
    }

    /// Adding potentially amplified coordinate to coordinate. Out-of-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d::new([1.0, 1.0]);
    /// let c1 = c0.add_out(1.0, &Coord2d{x: 10.0, y: 10.0});
    /// assert!((c1.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.y - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add_out(&self, a: f64, c: &Coord2d) -> Self
    {
        Coord2d{
            x: self.x + a * c.x,
            y: self.y + a * c.y }
    }

    /// Computing 2d coordinate using linear combination of two coordinates. In-place function.
    ///
    /// * `a` - First scalar coefficient in combination applied on calling instance.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.;
    /// let mut c0 = Coord2d::new([1.0, 3.0]);
    /// let b = -2.;
    /// let c1 = Coord2d::new([10., 10.]);
    ///
    /// c0.mlt_add_in(a, b, &c1);
    ///
    /// assert!((c0.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c0.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn mlt_add_in(&mut self, a: f64, b: f64, c: &Coord2d) -> &mut Self
    {
        self.x = a * self.x + b * c.x;
        self.y = a * self.y + b * c.y;
        self
    }

    /// Creating 2d coordinate using linear combination of two coordinates. Out-of-place function.
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.;
    /// let c0 = Coord2d::new([1.0, 3.0]);
    /// let b = -2.;
    /// let c1 = Coord2d::new([10., 10.]);
    ///
    /// let c2 = c0.mlt_add_out(a, b, &c1);
    ///
    /// assert!((c2.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c2.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn mlt_add_out(&self, a: f64, b: f64, c: &Coord2d) -> Self
    {
        Coord2d {
            x: a * self.x + b * c.x,
            y: a * self.y + b * c.y }
    }

    /// Comparing a coordinate with another one using a fixed epsilon. The comparison is done by
    /// computing the square norme of the difference between the two coordinate.
    ///
    /// * `c` - Coordinate to compare with.
    /// * `eps` - Threshold used for fixed-epsilon floating point comparison.
    ///
    /// # Example
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d::new([1.0, 3.4]);
    /// let c1 = Coord2d::new([6.0, -13.0]);
    /// let c2 = c0.clone();
    ///
    /// assert!(!c0.equals(&c1, GEOMETRICAL_TOLERANCE));
    /// assert!(c0.equals(&c2, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn equals(&self, c: &Coord2d, eps: f64) -> bool
    {
        self.add_out(-1.0, c).sq_norm() < eps
    }

    /// Computing square norm of a 2d coordinate.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d::new([2.0, 2.0]);
    /// assert!((c0.sq_norm() - 8.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn sq_norm(&self) -> f64
    {
        self.x * self.x + self.y * self.y
    }

    /// Computing norm of a 2d coordinate.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d::new([2.0, 2.0]);
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
    /// * `coords` - associated coordinates.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::new([1.0, 0.0]);
    /// assert!(p.coords.equals(&Coord2d::new([1.0, 0.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(coords: [f64; 2]) -> Self
    {
        Pnt2d{ coords: Coord2d::new(coords) }
    }

    /// Computing distance to another 2d points.
    ///
    /// * `q` - Input 2d point to compute the distance from.
    ///
    /// # Example 0
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new([0.0, 1.0]);
    ///
    /// assert!((p.distance_to(&q) - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// # Example 1
    /// ```
    /// use mersh::base::*;
    /// let p = Pnt2d::default();
    /// assert!(p.distance_to(&p) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn distance_to(&self, q: &Pnt2d) -> f64
    {
        self.coords.add_out(-1.0, &q.coords).norm()
    }

    /// Creating new point by applying translation defined from an input vector.
    ///
    /// * `v` - Input vector used to create point.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::new([1.45, 3.0]);
    /// let v = Vec2d::new([-0.1, 4.09]);
    /// let q = p.translate_by(&v);
    ///
    /// assert!((q.coords.x - p.coords.x - v.coords.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.y - p.coords.y - v.coords.y).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn translate_by(&self, v: &Vec2d) -> Self
    {
        Pnt2d { coords: self.coords.add_out(1.0, &v.coords) }
    }

    /// Creating a vector pointing to an input point.
    ///
    /// * `p` - The point to point to.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt2d::default();
    /// let q = Pnt2d::new([1.0, 0.0]);
    ///
    /// let v0 = p.to(&q);
    /// let v1 = q.to(&p);
    ///
    /// assert!(v0.coords.equals(&Coord2d { x: 1.0, y: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// assert!(v1.coords.equals(&Coord2d { x: -1.0, y: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn to(&self, p: &Pnt2d) -> Vec2d
    {
        Vec2d { coords: p.coords.add_out(-1.0, &self.coords) }
    }
}

impl Vec2d {
    /// Creating new vector from coordinates.
    ///
    /// * `coords` - associated coordinates.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec2d::new([1.0, 0.0]);
    /// assert!(v.coords.equals(&Coord2d { x: 1.0, y: 0.0}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(coords: [f64; 2]) -> Self
    {
        Vec2d{ coords: Coord2d::new(coords) }
    }
}

impl Dir2d {
    /// Creating orthogonal direction so that the cross product between the directions belongs to
    /// positive half space. Out-of-place function.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// use std::f64;
    ///
    /// let d0 = Dir2d { coords: Coord2d { x: (0.5 as f64).sqrt(), y: (0.5 as f64).sqrt() } };
    /// let d1 = d0.orthogonal_out();
    ///
    /// assert!(d1.coords.equals(&Coord2d { x: -(0.5 as f64).sqrt(), y: (0.5 as f64).sqrt()}, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn orthogonal_out(&self) -> Self
    {
        Dir2d { coords: Coord2d { x: -self.coords.y, y: self.coords.x } }
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining 3d coordinates.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Coord3d {
    /// First coordinate.
    pub x: f64,
    /// Second coordinate.
    pub y: f64,
    /// Third coordinate.
    pub z: f64,
}

/// Structure for defining 3d points.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Pnt3d {
    /// Coordinates associated to the point.
    pub coords: Coord3d,
}

/// Structure for defining 3d vectors.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Vec3d {
    /// Coordinates associated to the vector.
    pub coords: Coord3d,
}

/// Structure for defining 3d directions (i.e. unit vectors).
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Dir3d {
    /// Coordinates associated to the direction.
    pub coords: Coord3d,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl std::fmt::Display for Coord3d {
    /// Implementing display for 3d coordinates. By default the number of decimal is set to 6.
    ///
    /// * `formatter` - input reference to formatter.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    /// let coords = Coord3d::new([1.0, 0.0, 3.0]);
    /// let formatted_coords = format!("{}", coords);
    ///
    /// assert_eq!("(1.000000, 0.000000, 3.000000)", formatted_coords);
    /// ```
    ///
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
    }
}

impl Coord3d {
    /// Creating new coordinates.
    ///
    /// * `xyz` - associated coordinates values.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let coords = Coord3d::new([1.0, 0.0, 3.0]);
    /// assert!((coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((coords.y - 0.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((coords.z - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn new(xyz: [f64; 3]) -> Self
    {
        Coord3d { x: xyz[0], y: xyz[1], z: xyz[2] }
    }

    /// Amplifying coordinates by a scalar coefficient. In-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord3d::new([1.0, 2.0, 3.0]);
    /// c.amplify_in(3.0);
    /// assert!((c.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 6.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.z - 9.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify_in(&mut self, a: f64) -> &mut Self
    {
        self.x *= a;
        self.y *= a;
        self.z *= a;
        self
    }

    /// Amplifying coordinates by a scalar coefficient. Out-of-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d::new([1.0, 2.0, 3.0]);
    /// let c1 = c0.amplify_out(3.0);
    /// assert!((c1.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.y - 6.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.z - 9.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn amplify_out(&self, a: f64) -> Self
    {
        Coord3d {
            x: a * self.x,
            y: a * self.y,
            z: a * self.z }
    }

    /// Adding potentially amplified coordinate to coordinate. In-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord3d::new([1.0, 2.0, 3.0]);
    /// c.add_in(1.0, &Coord3d::new([10.0, 10.0, 10.0]));
    /// assert!((c.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.y - 12.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c.z - 13.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add_in(&mut self, a: f64, c: &Coord3d) -> &mut Self
    {
        self.x += a * c.x;
        self.y += a * c.y;
        self.z += a * c.z;
        self
    }

    /// Adding potentially amplified coordinate to coordinate. Out-of-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d::new([1.0, 2.0, 3.0]);
    /// let c1 = c0.add_out(1.0, &Coord3d::new([10.0, 10.0, 10.0]));
    /// assert!((c1.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.y - 12.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c1.z - 13.0).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn add_out(&self, a: f64, c: &Coord3d) -> Self
    {
        Coord3d {
            x: self.x + a * c.x,
            y: self.y + a * c.y,
            z: self.z + a * c.z }
    }

    /// Computing 3d coordinate using linear combination of two coordinates. In-place function.
    ///
    /// * `a` - First scalar coefficient in combination applied on calling instance.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.;
    /// let mut c0 = Coord3d::new([1.0, 3.0, 5.0]);
    /// let b = -2.;
    /// let c1 = Coord3d::new([10.0, 10.0, 10.0]);
    ///
    /// c0.mlt_add_in(a, b, &c1);
    ///
    /// assert!((c0.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c0.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(c0.z.abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn mlt_add_in(&mut self, a: f64, b: f64, c: &Coord3d) -> &mut Self
    {
        self.x = a * self.x + b * c.x;
        self.y = a * self.y + b * c.y;
        self.z = a * self.z + b * c.z;
        self
    }

    /// Creating 3d coordinate using linear combination of two coordinates. Out-of-place function.
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let a = 4.;
    /// let c0 = Coord3d::new([1.0, 3.0, 5.0]);
    /// let b = -2.;
    /// let c1 = Coord3d::new([10.0, 10.0, 10.0]);
    ///
    /// let c2 = c0.mlt_add_out(a, b, &c1);
    ///
    /// assert!((c2.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((c2.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!(c2.z.abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn mlt_add_out(&self, a: f64, b: f64, c: &Coord3d) -> Self
    {
        Coord3d {
            x: a * self.x + b * c.x,
            y: a * self.y + b * c.y,
            z: a * self.z + b * c.z }
    }

    /// Comparing a coordinate with another one using a fixed epsilon. The comparison is done by
    /// computing the square norme of the difference between the two coordinate.
    ///
    /// * `c` - Coordinate to compare with.
    /// * `eps` - Thershold used for fixed-epsilon floating point comparison.
    ///
    /// # Example
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d::new([1.0, 3.4, 2.0]);
    /// let c1 = Coord3d::new([6.0, -13.0, -1.98]);
    /// let c2 = c0.clone();
    ///
    /// assert!(!c0.equals(&c1, GEOMETRICAL_TOLERANCE));
    /// assert!(c0.equals(&c2, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn equals(&self, c: &Coord3d, eps: f64) -> bool
    {
        self.add_out(-1.0, c).sq_norm() < eps
    }

    /// Computing square norm of a 3d coordinate.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d::new([2.0, 2.0, 1.0]);
    /// assert!((c0.sq_norm() - 9.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn sq_norm(&self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computing norm of a 3d coordinate.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord3d::new([2.0, 2.0, 23.1]);
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
    /// * `coords` - associated coordinates.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::new([1.0, 0.0, 2.0]);
    /// assert!(p.coords.equals(&Coord3d::new([1.0, 0.0, 2.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(coords: [f64; 3]) -> Self
    {
        Pnt3d{ coords: Coord3d::new(coords) }
    }

    /// Computing distance to another 3d points.
    ///
    /// * `q` - Input 3d point to compute the distance from.
    ///
    /// # Example 0
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new([0.0, 1.0, 0.0]);
    ///
    /// assert!((p.distance_to(&q) - 1.0) < GEOMETRICAL_TOLERANCE);
    /// ```
    ///
    /// # Example 1
    /// ```
    /// use mersh::base::*;
    /// let p = Pnt3d::new([2.4, -1.9, 1.0]);
    /// assert!(p.distance_to(&p) < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn distance_to(&self, q: &Pnt3d) -> f64
    {
        self.coords.add_out(-1.0, &q.coords).norm()
    }

    /// Creating new point by applying translation defined from an input vector.
    ///
    /// * `v` - Input vector used to create point.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::new([1.45, 3.0, 2.0]);
    /// let v = Vec3d::new([-0.1, 4.09, 0.98]);
    /// let q = p.translate_by(&v);
    ///
    /// assert!((q.coords.x - p.coords.x - v.coords.x).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.y - p.coords.y - v.coords.y).abs() < GEOMETRICAL_TOLERANCE);
    /// assert!((q.coords.z - p.coords.z - v.coords.z).abs() < GEOMETRICAL_TOLERANCE);
    /// ```
    pub fn translate_by(&self, v: &Vec3d) -> Self
    {
        Pnt3d { coords: self.coords.add_out(1.0, &v.coords) }
    }

    /// Creating a vector pointing to an input point.
    ///
    /// * `p` - The point to point to.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new([1.0, 0.0, 0.0]);
    ///
    /// let v0 = p.to(&q);
    /// let v1 = q.to(&p);
    ///
    /// assert!(v0.coords.equals(&Coord3d { x: 1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// assert!(v1.coords.equals(&Coord3d { x: -1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn to(&self, p: &Pnt3d) -> Vec3d
    {
        Vec3d { coords: p.coords.add_out(-1.0, &self.coords) }
    }
}

impl Vec3d {
    /// Creating new vector from coordinates.
    ///
    /// * `coords` - Associated coordinate.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let v = Vec3d::new([1.0, 0.0, 0.0]);
    /// assert!(v.coords.equals(&Coord3d::new([1.0, 0.0, 0.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn new(coords: [f64; 3]) -> Self
    {
        Vec3d{ coords: Coord3d::new(coords) }
    }

    /// Creating a vector by applying cross product. Out-of-place function.
    ///
    /// * `v` - Second vector used for cross product.
    ///
    /// # Example 0
    /// ```
    /// use mersh::base::*;
    ///
    /// let u = Vec3d::new([1.0, 0.0, 0.0]);
    /// let v = Vec3d::new([1.0, 2.0, 0.0]);
    /// let w = u.cross_out(&v);
    ///
    /// assert!(w.coords.equals(&Coord3d::new([0.0, 0.0, 2.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    ///
    /// # Example 1
    /// ```
    /// use mersh::base::*;
    ///
    /// let u = Vec3d::new([1.0, 0.0, 0.0]);
    /// let v = Vec3d::new([1.0, 2.0, 0.0]);
    /// let w = v.cross_out(&u);
    ///
    /// assert!(w.coords.equals(&Coord3d::new([0.0, 0.0, -2.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn cross_out(&self, v: &Vec3d) -> Vec3d
    {
        Vec3d::new([
            self.coords.y * v.coords.z - self.coords.z * v.coords.y,
            self.coords.z * v.coords.x - self.coords.x * v.coords.z,
            self.coords.x * v.coords.y - self.coords.y * v.coords.x
        ])
    }

    /// Creating new direction by normalizing the vector. Out-of-place function.
    ///
    /// # Example
    /// ```
    /// use mersh::base::*;
    ///
    /// let p = Pnt3d::default();
    /// let q = Pnt3d::new([0.0, 3.0, 0.0]);
    ///
    /// let u = p.to(&q);
    /// let v = q.to(&p);
    ///
    /// let d = u.normalize_out();
    /// let l = v.normalize_out();
    ///
    /// assert!(d.coords.equals(&Coord3d::new([0.0, 1.0, 0.0]), GEOMETRICAL_TOLERANCE));
    /// assert!(l.coords.equals(&Coord3d::new([0.0,-1.0, 0.0]), GEOMETRICAL_TOLERANCE));
    /// ```
    pub fn normalize_out(&self) -> Dir3d
    {
        let norm = self.coords.norm();
        Dir3d{ coords: self.coords.amplify_out(1.0 / norm) }
    }
}
