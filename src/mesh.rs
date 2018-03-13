extern crate std;

use super::base::*;
use super::elements::*;
use super::tag::*;
use super::views::*;
use std::vec::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Structure defining a 3d tagged mesh.
#[derive(Default, Serialize, Deserialize)]
pub struct Mesh3d {
    pub vertices: Vec<Pnt3d>,
    pub edges: Vec<Edge>,
    pub triangles: Vec<Tri>,
    pub quadrangles: Vec<Quad>,
    pub tetrahedra: Vec<Tet>,
    pub hexahedra: Vec<Hexa>,
    pub vertices_tags: TagSet,
    pub edges_tags: TagSet,
    pub triangles_tags: TagSet,
    pub quadrangles_tags: TagSet,
    pub tetrahedra_tags: TagSet,
    pub hexahedra_tags: TagSet,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// 3D implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Mesh3d {
    /// Creating a tagged vertex from coordinates & tag name.
    ///
    /// * `point` - Point to add in the mesh.
    /// * `name` - Tag name.
    ///
    pub fn push_tagged_vertex(&mut self, point: Pnt3d, name: &str)
    {
        push_tagged_element(&mut self.vertices, &mut self.vertices_tags, point, name);
    }

    /// Creating an tagged edge in the mesh.
    ///
    /// * `edge` - Edge to add in the mesh.
    /// * `name` - Tag name.
    ///
    pub fn push_tagged_edge(&mut self, edge: Edge, name: &str)
    {
       push_tagged_element(&mut self.edges, &mut self.edges_tags, edge, name);
    }

    /// Creating a view to an edge in a mesh from the input edge itself.
    ///
    /// * `edge` - Edge in the mesh.
    ///
    pub fn get_edge_view<'a>(&'a self, edge: &Edge) -> EdgeView3d<'a>
    {
         EdgeView3d { points: get_two_vertices_view(&self.vertices, &edge.indexes) }
    }

    /// Creating an tagged triangle in the mesh.
    ///
    /// * `tri` - Triangle to add in the mesh.
    /// * `name` - tag name.
    ///
    pub fn push_tagged_triangle(&mut self, tri: Tri, name: &str)
    {
       push_tagged_element(&mut self.triangles, &mut self.triangles_tags, tri, name);
    }

    /// Making a view to a triangle in a mesh from the element itself.
    ///
    /// * `tri` - Triangle in the mesh.
    ///
    pub fn get_tri_view<'a>(&'a self, tri: &Tri) -> TriView3d<'a>
    {
        TriView3d { points: get_three_vertices_view(&self.vertices, &tri.indexes)}
    }

    /// Creating an tagged quadrangle in the mesh.
    ///
    /// * `quad` - Quadrangle to add in the mesh.
    /// * `name` - Tag name.
    ///
    pub fn push_tagged_quadrangle(&mut self, quad: Quad, name: &str)
    {
       push_tagged_element(&mut self.quadrangles, &mut self.quadrangles_tags, quad, name);
    }

    /// Making a view to a quadrangle in a mesh from the element itself.
    ///
    /// * `quad` - Quadrangle in the mesh.
    ///
    pub fn get_quad_view<'a>(&'a self, quad: &Quad) -> QuadView3d<'a>
    {
        QuadView3d { points: get_four_vertices_view(&self.vertices, &quad.indexes) }
    }

    /// Creating a tagged tetrahedron in the mesh.
    ///
    /// * `tet` - Tetrahedron to add in the mesh.
    /// * `name` - Tag name.
    ///
    pub fn push_tagged_tetrahedron(&mut self, tet: Tet, name: &str)
    {
       push_tagged_element(&mut self.tetrahedra, &mut self.tetrahedra_tags, tet, name);
    }

    /// Making a view to a tetrahedron in a mesh the element itself.
    ///
    /// * `tet` - Tetrahedron in the mesh.
    ///
    pub fn get_tet_view<'a>(&'a self, tet: &Tet) -> TetView3d<'a>
    {
       TetView3d { points: get_four_vertices_view(&self.vertices, &tet.indexes) }
    }

    /// Creating a tagged hexahedron in the mesh.
    ///
    /// * `hexa` - Hexahedron to add in the mesh.
    /// * `name` - Tag name.
    ///
    pub fn push_tagged_hexahedron(&mut self, hexa: Hexa, name: &str)
    {
       push_tagged_element(&mut self.hexahedra, &mut self.hexahedra_tags, hexa, name);
    }

    /// Making a view to a hexahedron in a mesh from the element itself.
    ///
    /// * `hexa` - Hexahedron in the mesh.
    ///
    pub fn get_hexa_view<'a>(&'a self, hexa: &Hexa) -> HexaView3d<'a>
    {
       HexaView3d { points: get_eight_vertices_view(&self.vertices, &hexa.indexes) }
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Private implementation methods.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

// Pushing an element into a vector of elements and registering its associated tag.
fn push_tagged_element<T>(elements: &mut Vec<T>, tags: &mut TagSet, element: T, name: &str)
{
    let idx = elements.len();
    elements.push(element);
    tags.register(name, idx);
}

// Extracting reference to vertices of a two vertices element.
fn get_two_vertices_view<'a, T>(vertices: &'a [T], indexes: &[usize; 2]) -> [&'a T; 2]
{
    [
        &vertices[indexes[0]], &vertices[indexes[1]]
    ]
}

// Extracting reference to vertices of a two vertices element.
fn get_three_vertices_view<'a, T>(vertices: &'a [T], indexes: &[usize; 3]) -> [&'a T; 3]
{
    [
        &vertices[indexes[0]], &vertices[indexes[1]], &vertices[indexes[2]]
    ]
}

// Extracting reference to vertices of a two vertices element.
fn get_four_vertices_view<'a, T>(vertices: &'a [T], indexes: &[usize; 4]) -> [&'a T; 4]
{
    [
        &vertices[indexes[0]],&vertices[indexes[1]],
        &vertices[indexes[2]],&vertices[indexes[3]]
    ]
}

// Extracting reference to vertices of a two vertices element.
fn get_eight_vertices_view<'a, T>(vertices: &'a [T], indexes: &[usize; 8]) -> [&'a T; 8]
{
    [
        &vertices[indexes[0]], &vertices[indexes[1]], &vertices[indexes[2]], &vertices[indexes[3]],
        &vertices[indexes[4]], &vertices[indexes[5]], &vertices[indexes[6]], &vertices[indexes[7]]
    ]
}
