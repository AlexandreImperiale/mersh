extern crate std;

/// Associated geometrical tolerance.
pub const GEOMETRICAL_TOLERANCE: f64 = 1e-12;

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
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
    }
}

impl Coord3d {
    /// Creating new coordinates.
    ///
    /// * `xyz` - associated coordinates values.
    ///
    pub fn new(xyz: [f64; 3]) -> Self
    {
        Coord3d { x: xyz[0], y: xyz[1], z: xyz[2] }
    }

    /// Amplifying coordinates by a scalar coefficient. In-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    pub fn amplify_in(&mut self, a: f64) -> &mut Self
    {
        self.x *= a; self.y *= a; self.z *= a; self
    }

    /// Amplifying coordinates by a scalar coefficient. Out-of-place function.
    ///
    /// * `a` - Scalar coefficient used for amplification.
    ///
    pub fn amplify_out(&self, a: f64) -> Self
    {
        Coord3d { x: a * self.x, y: a * self.y, z: a * self.z }
    }

    /// Adding potentially amplified coordinate to coordinate. In-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    pub fn add_in(&mut self, a: f64, c: &Coord3d) -> &mut Self
    {
        self.x += a * c.x; self.y += a * c.y; self.z += a * c.z; self
    }

    /// Adding potentially amplified coordinate to coordinate. Out-of-place function.
    ///
    /// * `a` - Coefficient applied on input coordinate.
    /// * `c` - Coordinate to add.
    ///
    pub fn add_out(&self, a: f64, c: &Coord3d) -> Self
    {
        Coord3d { x: self.x + a * c.x, y: self.y + a * c.y, z: self.z + a * c.z }
    }

    /// Computing 3d coordinate using linear combination of two coordinates. In-place function.
    ///
    /// * `a` - First scalar coefficient in combination applied on calling instance.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    pub fn mlt_add_in(&mut self, a: f64, b: f64, c: &Coord3d) -> &mut Self
    {
        self.x = a * self.x + b * c.x; self.y = a * self.y + b * c.y; self.z = a * self.z + b * c.z;
        self
    }

    /// Creating 3d coordinate using linear combination of two coordinates. Out-of-place function.
    ///
    /// * `a` - First scalar coefficient in combination.
    /// * `b` - Second scalar coefficient in combination.
    /// * `c` - Second coordinate in combination.
    ///
    pub fn mlt_add_out(&self, a: f64, b: f64, c: &Coord3d) -> Self
    {
        Coord3d { x: a * self.x + b * c.x, y: a * self.y + b * c.y, z: a * self.z + b * c.z }
    }

    /// Comparing a coordinate with another one using a fixed epsilon. The comparison is done by
    /// computing the square norm of the difference between the two coordinate.
    ///
    /// * `c` - Coordinate to compare with.
    /// * `eps` - Threshold used for fixed-epsilon floating point comparison.
    ///
    pub fn equals(&self, c: &Coord3d, eps: f64) -> bool
    {
        self.add_out(-1.0, c).sq_norm() < eps
    }

    /// Computing square norm of a 3d coordinate.
    ///
    pub fn sq_norm(&self) -> f64
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computing norm of a 3d coordinate.
    ///
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
    pub fn new(coords: [f64; 3]) -> Self
    {
        Pnt3d{ coords: Coord3d::new(coords) }
    }

    /// Computing distance to another 3d points.
    ///
    /// * `q` - Input 3d point to compute the distance from.
    ///
    pub fn distance_to(&self, q: &Pnt3d) -> f64
    {
        self.coords.add_out(-1.0, &q.coords).norm()
    }

    /// Creating new point by applying translation defined from an input vector.
    ///
    /// * `v` - Input vector used to create point.
    ///
    pub fn translate_by(&self, v: &Vec3d) -> Self
    {
        Pnt3d { coords: self.coords.add_out(1.0, &v.coords) }
    }

    /// Creating a vector pointing to an input point.
    ///
    /// * `p` - The point to point to.
    ///
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
    pub fn new(coords: [f64; 3]) -> Self
    {
        Vec3d{ coords: Coord3d::new(coords) }
    }

    /// Creating a vector by applying cross product. Out-of-place function.
    ///
    /// * `v` - Second vector used for cross product.
    ///
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
    pub fn normalize_out(&self) -> Dir3d
    {
        let norm = self.coords.norm();
        Dir3d{ coords: self.coords.amplify_out(1.0 / norm) }
    }
}
