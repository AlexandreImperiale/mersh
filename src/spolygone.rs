extern crate std;

use super::elements::*;
use super::mesh::*;
use super::topology::*;

// Data structure for defining simple polygone.
pub struct Spolygone2d {
    /// Associated mesh.
    pub mesh: Mesh2d,
}

impl Spolygone2d {
    /// Triangulating simple polygone - based upon the ear clipping algorithm.
    pub fn triangulate(&mut self)
    {
        // To Do.
    }
}
