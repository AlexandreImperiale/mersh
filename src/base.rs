/// Structure for defining 2d coordinates.
#[derive(Clone)]
pub struct Coord2d {
    /// First coordinate.
    pub x: f64,
    /// Second coordinate.
    pub y: f64,
}

/// Structure for defining 2d points.
#[derive(Clone)]
pub struct Pnt2d {
    /// Coordinates associated to the point.
    pub coords: Coord2d,
}

/// Structure for defining 2d vectors.
#[derive(Clone)]
pub struct Vec2d {
    /// Coordinates associated to the vector.
    pub coords: Coord2d,
}

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
    /// assert!((c.x - 3.0).abs() < 1e-10);
    /// assert!((c.y - 3.0).abs() < 1e-10);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Coord2d
    {
        self.x *= a; self.y *= a; self
    }

    /// Applying linear combination to coordinates,
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Coordinates appearing in combination.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let mut c = Coord2d{x: 1.0, y: 1.0};
    /// c.add(1.0, &Coord2d{x: 10.0, y: 10.0});
    /// assert!((c.x - 11.0).abs() < 1e-10);
    /// assert!((c.y - 11.0).abs() < 1e-10);
    /// ```
    pub fn add(&mut self, a: f64, c: &Coord2d) -> &mut Coord2d
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
    /// assert!((c2.x - a * c0.x - b * c1.x).abs() < 1e-10);
    /// assert!((c2.y - a * c0.y - b * c1.y).abs() < 1e-10);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 1.0, y: 3.4 };
    /// let c1 = Coord2d::mlt_add(1.0, &c0, -1.0, &c0);
    ///
    /// assert!(c1.x.abs() < 1e-10);
    /// assert!(c1.y.abs() < 1e-10);
    ///```
    pub fn mlt_add(a: f64, c0: &Coord2d, b: f64, c1: &Coord2d) -> Coord2d
    {
        Coord2d { x: a * c0.x + b * c1.x, y: a * c0.y + b * c1.y }
    }

    /// Comparing a coordinate with another one using a fixed epsilon.
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
    /// assert!(!c0.equals(&c1, 1e-10));
    /// assert!(c0.equals(&c2, 1e-10));
    /// ```
    pub fn equals(&self, c: &Coord2d, eps: f64) -> bool
    {
        let delta = Coord2d::mlt_add(1.0, &self, -1.0, &c);
        delta.x.abs() < eps && delta.y.abs() < eps
    }

    /// Computing square norm of a 2d coordinate.
    ///
    /// # Examples
    /// ```
    /// use mersh::base::*;
    ///
    /// let c0 = Coord2d { x: 2.0, y: 2.0 };
    /// assert!((c0.sq_norm() - 8.0) < 1e-10);
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
    /// assert!((c0.norm() - c0.sq_norm().sqrt()) < 1e-10);
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
    /// assert!(p.coords.equals(&Coord2d { x: 1.0, y: 0.0}, 1e-10));
    /// ```
    pub fn new(x: f64, y: f64) -> Pnt2d
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
    /// let p = Pnt2d::new(0.0, 0.0);
    /// let q = Pnt2d::new(0.0, 1.0);
    ///
    /// assert!((p.distance_to(&q) - 1.0) < 1e-10);
    /// ```
    ///
    /// ```
    /// use mersh::base::*;
    /// let p = Pnt2d::new(0.0, 0.0);
    /// assert!(p.distance_to(&p) < 1e-10);
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
    /// assert!((q.coords.x - p.coords.x - v.coords.x).abs() < 1e-10);
    /// assert!((q.coords.y - p.coords.y - v.coords.y).abs() < 1e-10);
    /// ```
    pub fn add(&self, v: &Vec2d) -> Pnt2d
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
    /// let p = Pnt2d::new(0.0, 0.0);
    /// let q = Pnt2d::new(1.0, 0.0);
    ///
    /// let v0 = p.to(&q);
    /// let v1 = q.to(&p);
    ///
    /// assert!(v0.coords.equals(&Coord2d { x: 1.0, y: 0.0 }, 1e-10));
    /// assert!(v1.coords.equals(&Coord2d { x: -1.0, y: 0.0 }, 1e-10));
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
    /// assert!(v.coords.equals(&Coord2d { x: 1.0, y: 0.0}, 1e-10));
    /// ```
    pub fn new(x: f64, y: f64) -> Vec2d
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
    /// assert!((v.amplify(a).coords.norm() - a).abs() < 1e-10);
    /// ```
    pub fn amplify(&mut self, a: f64) -> &mut Vec2d
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
    /// assert!((v.norm() - 1.0) < 1e-10);
    /// ```
    pub fn norm(&self) -> f64
    {
        self.coords.norm()
    }
}
