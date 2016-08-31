extern crate mersh;

// Helper function used to build triangular mesh of the unit square.
fn make_square() -> mersh::mesh::Mesh2d
{
    let mut mesh = mersh::mesh::Mesh2d::new();

    // Adding points.
    mesh.add_vertex(0., 0., 0)
        .add_vertex(1., 0., 0)
        .add_vertex(1., 1., 0)
        .add_vertex(0., 1., 0);

    // Adding triangles.
    mesh.add_tri(0, 1, 2, 0)
        .add_tri(2, 3, 0, 0);

    return mesh;
}

#[test]
fn square_view_triangles() {

    // Building mesh.
    let mut mesh = make_square();

    // Computing area of first and second triangle.
    assert!((mesh.view_tri(0).get_area() - 0.5).abs() < 1e-10);
    assert!((mesh.view_tri(1).get_area() - 0.5).abs() < 1e-10);

    // Enlarging mesh points.
    for v in mesh.vertices.iter_mut() { v.point.coords.amplify(10.0); }

    // Recomputing area of first and second triangle.
    assert!((mesh.view_tri(0).get_area() - 50.0).abs() < 1e-10);
    assert!((mesh.view_tri(1).get_area() - 50.0).abs() < 1e-10);
}

#[test]
fn square_topology() {

    // Building mesh.
    let mesh = make_square();

    // Extracting mesh topology.
    let mut topology = mersh::topology::Topology::new(&mesh);

    // Building vertices and triangles topology.
    topology.build_vertices()
            .build_triangles();

    // Querying topology of first vertex.
    let ref v0 = &topology.vertices[0];
    assert!(v0.incident_tris[0].tri_index == 0);
    assert!(v0.incident_tris[0].connecting_vertex as usize == 0);
    assert!(v0.incident_tris[1].tri_index == 1);
    assert!(v0.incident_tris[1].connecting_vertex as usize == 2);
}
