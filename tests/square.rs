extern crate mersh;

use mersh::mesh::*;
use mersh::elements::*;

// Helper function used to build a mesh of the unit square.
fn make_square() -> mersh::mesh::Mesh2d
{
    let mut mesh = mersh::mesh::Mesh2d::new();

    // Adding points.
    mesh.vertices.push(Vertex2d::new_untagged([0., 0.]));
    mesh.vertices.push(Vertex2d::new_untagged([1., 0.]));
    mesh.vertices.push(Vertex2d::new_untagged([1., 1.]));
    mesh.vertices.push(Vertex2d::new_untagged([0., 1.]));

    // Adding triangles.
    mesh.triangles.push(Tri::new_untagged([0, 1, 2]));
    mesh.triangles.push(Tri::new_untagged([2, 3, 0]));

    // Adding quadrangle.
    mesh.quadrangles.push(Quad::new_untagged([0, 1, 3, 2]));

    return mesh;
}

#[test]
fn square_view_triangles() {

    // Building mesh.
    let mut mesh = make_square();

    // Computing area of first and second triangle.
    assert!((mesh.get_tri_view(0).get_area() - 0.5).abs() < 1e-10);
    assert!((mesh.get_tri_view(1).get_area() - 0.5).abs() < 1e-10);

    // Enlarging mesh points.
    for v in mesh.vertices.iter_mut() { v.point.coords.amplify_in(10.0); }

    // Recomputing area of first and second triangle.
    assert!((mesh.get_tri_view(0).get_area() - 50.0).abs() < 1e-10);
    assert!((mesh.get_tri_view(1).get_area() - 50.0).abs() < 1e-10);
}
