
/// Amplifying coordinates by a scalar coefficient.
pub fn amplify(&mut self, a: f64) -> &mut Coord2d
{
    // ... //
}

/// Applying linear combination to coordinates,
pub fn add(&mut self, a: f64, c: &Coord2d) -> &mut Coord2d
{
    // ... //
}

/// Creating 2d coordinate using linear combination of two coordinates.
pub fn mlt_add(a: f64, c0: &Coord2d, b: f64, c1: &Coord2d) -> Coord2d
{
    // ... //
}

/// Comparing a coordinate with another one using a fixed epsilon.
pub fn equals(&self, c: &Coord2d, eps: f64) -> bool
{
    // ... //
}
