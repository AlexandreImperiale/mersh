extern crate std;

use super::base::*;
use super::elements::*;
use super::mesh::*;
use std::vec::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Definition of command data structure.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Definition of commands to be applied on a mesh.
#[derive(Serialize, Deserialize)]
pub enum In {
    /// Accessing 2d vertex in mesh.
    GetVertex2d(usize),
    /// Accessing 3d vertex in mesh.
    GetVertex3d(usize),
    /// Pushing 2d vertex into mesh.
    PushVertex2d([f64; 2]),
    /// Pushing 2d vertex tagged into mesh.
    PushTaggedVertex2d([f64; 2], String),
    /// Pushing 3d vertex into mesh.
    PushVertex3d([f64; 3]),
    /// Pushing "d vertex tagged into mesh.
    PushTaggedVertex3d([f64; 3], String),
    /// Pushing edge into mesh.
    PushEdge([usize; 2]),
    /// Pushing tagged edge into mesh.
    PushTaggedEdge([usize; 2], String),
    /// Pushing triangle into mesh.
    PushTri([usize; 3]),
    /// Pushing tagged triangle into mesh.
    PushTaggedTri([usize; 3], String),
    /// Pushing quadrangle into mesh.
    PushQuad([usize; 4]),
    /// Pushing tagged quadrangle into mesh.
    PushTaggedQuad([usize; 4], String),
}

/// Definition of command outputs.
#[derive(Serialize, Deserialize)]
pub enum Out {
    /// Accessed 2d vertex in mesh.
    GotVertex2d([f64; 2]),
   /// Accessed 3d vertex in mesh.
    GotVertex3d([f64; 3]),
    /// Pushed 2d vertex into mesh.
    PushedVertex2d(),
    /// Pushed 2d vertex tagged into mesh.
    PushedTaggedVertex2d(),
    /// Pushed 3d vertex into mesh.
    PushedVertex3d(),
    /// Pushed "d vertex tagged into mesh.
    PushedTaggedVertex3d(),
    /// Pushed edge into mesh.
    PushedEdge(),
    /// Pushed tagged edge into mesh.
    PushedTaggedEdge(),
    /// Pushed triangle into mesh.
    PushedTri(),
    /// Pushed tagged triangle into mesh.
    PushedTaggedTri(),
    /// Pushed quadrangle into mesh.
    PushedQuad(),
    /// Pushed tagged quadrangle into mesh.
    PushedTaggedQuad(),
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Applying inputs & outputs.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Applying a list of commends, in order, on a 2d mesh.
///
/// * `inputs` - Set of input commands.
///
/// Panics if one of the element in `inputs` is not supported.
///
/// # Example
/// ```
/// use mersh::base::*;
/// use mersh::cmd::*;
/// use mersh::mesh::*;
///
/// let mut mesh = Mesh2d::default();
/// let outputs = apply_commands_2d(&mut mesh, &vec![In::PushVertex2d([1.0, 2.0]), In::GetVertex2d(0)]);
///
/// match outputs[1] {
///     Out::GotVertex2d(coords) => {
///         assert!((coords[0] - 1.0).abs() < GEOMETRICAL_TOLERANCE);
///         assert!((coords[1] - 2.0).abs() < GEOMETRICAL_TOLERANCE);
///     },
///     _ => assert!(false)
/// }
/// ```
pub fn apply_commands_2d(mesh: &mut Mesh2d, inputs: &[In]) -> Vec<Out>
{
    let mut outputs : Vec<Out> = Vec::with_capacity(inputs.len());
    for input in inputs { outputs.push(apply_command_2d(mesh, input)) }
    outputs
}

/// Applying a list of commends, in order, on a 3d mesh.
///
/// * `inputs` - Set of input commands.
///
/// Panics if one of the element in `inputs` is not supported.
///
/// # Example
/// ```
/// use mersh::base::*;
/// use mersh::cmd::*;
/// use mersh::mesh::*;
///
/// let mut mesh = Mesh3d::default();
/// let outputs = apply_commands_3d(&mut mesh, &vec![In::PushVertex3d([1.0, 2.0, 3.0]), In::GetVertex3d(0)]);
///
/// match outputs[1] {
///     Out::GotVertex3d(coords) => {
///         assert!((coords[0] - 1.0).abs() < GEOMETRICAL_TOLERANCE);
///         assert!((coords[1] - 2.0).abs() < GEOMETRICAL_TOLERANCE);
///         assert!((coords[2] - 3.0).abs() < GEOMETRICAL_TOLERANCE);
///     },
///     _ => assert!(false)
/// }
/// ```
pub fn apply_commands_3d(mesh: &mut Mesh3d, inputs: &[In]) -> Vec<Out>
{
    let mut outputs : Vec<Out> = Vec::with_capacity(inputs.len());
    for input in inputs { outputs.push(apply_command_3d(mesh, input)) }
    outputs
}

/// Applying a command on a 2d mesh.
///
/// * `input` - Input command.
///
/// Panics if `input` is not supported.
///
/// # Example
/// ```
/// use mersh::base::*;
/// use mersh::cmd::*;
/// use mersh::mesh::*;
///
/// let mut mesh = Mesh2d::default();
/// apply_command_2d(&mut mesh, &In::PushVertex2d([1.0, 2.0]));
///
/// assert!((mesh.vertices[0].coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
/// assert!((mesh.vertices[0].coords.y - 2.0).abs() < GEOMETRICAL_TOLERANCE);
/// ```
pub fn apply_command_2d(mesh: &mut Mesh2d, input: &In) -> Out
{
    match *input {
        In::GetVertex2d(index) => { let coords = &mesh.vertices[index].coords; Out::GotVertex2d([coords.x, coords.y]) },
        In::PushVertex2d(coords) => { mesh.vertices.push(Pnt2d::new(coords)); Out::PushedVertex2d() },
        In::PushTaggedVertex2d(coords, ref name) => { mesh.push_tagged_vertex(Pnt2d::new(coords), name); Out::PushedTaggedVertex2d() },
        In::PushEdge(indexes) => { mesh.edges.push(Edge::new(indexes)); Out::PushedEdge() },
        In::PushTaggedEdge(indexes, ref name) => { mesh.push_tagged_edge(Edge::new(indexes), name); Out::PushedTaggedEdge() },
        In::PushTri(indexes) => { mesh.triangles.push(Tri::new(indexes)); Out::PushedTri() },
        In::PushTaggedTri(indexes, ref name) => { mesh.push_tagged_triangle(Tri::new(indexes), name); Out::PushedTaggedTri() },
        In::PushQuad(indexes) => { mesh.quadrangles.push(Quad::new(indexes)); Out::PushedQuad() },
        In::PushTaggedQuad(indexes, ref name) => { mesh.push_tagged_quadrangle(Quad::new(indexes), name); Out::PushedTaggedQuad() },
        _ => panic!("Unsupported command.")
    }
}

/// Applying a command on a 3d mesh.
///
/// * `cmd` - Input command.
///
/// Panics if `cmd` is not supported.
///
/// # Example
/// ```
/// use mersh::base::*;
/// use mersh::cmd::*;
/// use mersh::mesh::*;
///
/// let mut mesh = Mesh3d::default();
/// apply_command_3d(&mut mesh, &In::PushVertex3d([1.0, 2.0, 5.6]));
///
/// assert!((mesh.vertices[0].coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
/// assert!((mesh.vertices[0].coords.y - 2.0).abs() < GEOMETRICAL_TOLERANCE);
/// assert!((mesh.vertices[0].coords.z - 5.6).abs() < GEOMETRICAL_TOLERANCE);
/// ```
pub fn apply_command_3d(mesh: &mut Mesh3d, input: &In) -> Out
{
    match *input {
        In::GetVertex3d(index) => { let coords = &mesh.vertices[index].coords; Out::GotVertex3d([coords.x, coords.y, coords.z]) },
        In::PushVertex3d(coords) => { mesh.vertices.push(Pnt3d::new(coords)); Out::PushedVertex3d() },
        In::PushTaggedVertex3d(coords, ref name) => { mesh.push_tagged_vertex(Pnt3d::new(coords), name); Out::PushedTaggedVertex3d()},
        In::PushEdge(indexes) => { mesh.edges.push(Edge::new(indexes)); Out::PushedEdge() },
        In::PushTaggedEdge(indexes, ref name) => { mesh.push_tagged_edge(Edge::new(indexes), name); Out::PushedTaggedEdge() },
        In::PushTri(indexes) => { mesh.triangles.push(Tri::new(indexes)); Out::PushedTri() },
        In::PushTaggedTri(indexes, ref name) => { mesh.push_tagged_triangle(Tri::new(indexes), name); Out::PushedTaggedTri() },
        In::PushQuad(indexes) => { mesh.quadrangles.push(Quad::new(indexes)); Out::PushedQuad() },
        In::PushTaggedQuad(indexes, ref name) => { mesh.push_tagged_quadrangle(Quad::new(indexes), name); Out::PushedTaggedQuad() },
        _ => panic!("Unsupported command.")
    }
}

