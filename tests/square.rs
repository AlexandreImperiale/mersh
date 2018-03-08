extern crate mersh;

use mersh::base::*;
use mersh::elements::*;

// Helper function used to build a mesh of the unit square.
fn make_square() -> mersh::mesh::Mesh2d
{
    let mut mesh = mersh::mesh::Mesh2d::default();

    // Adding points.
    mesh.vertices.push(Pnt2d::new([0., 0.]));
    mesh.vertices.push(Pnt2d::new([1., 0.]));
    mesh.vertices.push(Pnt2d::new([1., 1.]));
    mesh.vertices.push(Pnt2d::new([0., 1.]));

    // Adding triangles.
    mesh.triangles.push(Tri::new([0, 1, 2]));
    mesh.triangles.push(Tri::new([2, 3, 0]));

    // Adding quadrangle.
    mesh.quadrangles.push(Quad::new([0, 1, 3, 2]));

    return mesh;
}

#[test]
fn square_view_triangles() {

    // Building mesh.
    let mut mesh = make_square();

    // Computing area of first and second triangle.
    assert!((mesh.get_tri_view(&mesh.triangles[0]).get_area() - 0.5).abs() < 1e-10);
    assert!((mesh.get_tri_view(&mesh.triangles[1]).get_area() - 0.5).abs() < 1e-10);

    // Computing global area covered by the triangles.
    let mut area = 0.;
    for tri in mesh.triangles.iter() { area += mesh.get_tri_view(tri).get_area(); }
    assert!((area - 1.0).abs() < 1e-10);

    // Enlarging mesh points.
    for point in mesh.vertices.iter_mut() { point.coords.amplify_in(10.0); }

    // Recomputing area of first and second triangle.
    assert!((mesh.get_tri_view(&mesh.triangles[0]).get_area() - 50.0).abs() < 1e-10);
    assert!((mesh.get_tri_view(&mesh.triangles[1]).get_area() - 50.0).abs() < 1e-10);
}

