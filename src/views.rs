use super::base::*;
use super::elements::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure for defining a 3d edge view.
pub struct EdgeView3d<'a> {
    /// Reference to vertices of the edge.
    pub points: [&'a Pnt3d; 2]
}

/// Structure for defining a 3d tri view.
pub struct TriView3d<'a> {
    /// Reference to vertices of the triangle.
    pub points: [&'a Pnt3d; 3]
}

/// Structure for defining a 3d quad view.
pub struct QuadView3d<'a> {
    /// Reference to vertices of the quadrangle.
    pub points: [&'a Pnt3d; 4]
}

/// Structure for defining a tetrahedron view.
pub struct TetView3d<'a> {
    /// Reference to vertices of the tetrahedron.
    pub points: [&'a Pnt3d; 4]
}

/// Structure for defining a hexahedron view.
pub struct HexaView3d<'a> {
    /// Reference to vertices of the hexahedron.
    pub points: [&'a Pnt3d; 8]
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl<'a> EdgeView3d<'a> {
    /// Computing length of a 3d view to an edge in a mesh.
    ///
    pub fn get_length(&self) -> f64
    {
        self.points[0].to(self.points[1]).coords.norm()
    }
}

impl<'a> TriView3d<'a> {
    /// Computing normal vector associated to a triangle.
    ///
    pub fn get_normal(&self) -> Dir3d
    {
        let u : Vec3d = self.points[0].to(self.points[1]);
        let v : Vec3d = self.points[0].to(self.points[2]);
        u.cross_out(&v).normalize_out()
    }

    /// Accessing view to a local edge in a triangle
    ///
    /// * `edge_name` - Local name of the edge in the triangle.
    ///
    pub fn get_edge_view<'b>(&'b self, edge_name: EdgeInTri) -> EdgeView3d<'a>
    {
        match edge_name {
            EdgeInTri::Edge01 => EdgeView3d{ points: [self.points[0], self.points[1]] },
            EdgeInTri::Edge12 => EdgeView3d{ points: [self.points[1], self.points[2]] },
            EdgeInTri::Edge20 => EdgeView3d{ points: [self.points[2], self.points[0]] },
        }
    }
}