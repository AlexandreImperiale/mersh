extern crate mersh;

mod mesh {

    use mersh::base::*;
    use mersh::elements::*;
    use mersh::mesh::*;

    #[test]
    fn push_tagged_vertex() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0.1, 2.6, -0.1]));
        mesh.push_tagged_vertex(Pnt3d::new([0.2, 1.6, 0.3]), &name);

        assert!((mesh.vertices[1].coords.x - 0.2).abs() < GEOMETRICAL_TOLERANCE);
        assert!((mesh.vertices[1].coords.y - 1.6).abs() < GEOMETRICAL_TOLERANCE);
        assert!((mesh.vertices[1].coords.z - 0.3).abs() < GEOMETRICAL_TOLERANCE);

        match mesh.vertices_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 1); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn push_tagged_edge() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0.0, 0.0, 2.0]));
        mesh.vertices.push(Pnt3d::new([0.0, 1.0, 2.0]));
        mesh.push_tagged_edge(Edge::new([0, 1]), &name);

        match mesh.edges_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 0); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn get_edge_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 2.0]));
        mesh.vertices.push(Pnt3d::new([1., 0., 1.5]));
        mesh.edges.push(Edge::new([0, 1]));

        let e = mesh.get_edge_view(&mesh.edges[0]);
        assert!((e.points[1].coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((e.points[1].coords.y - 0.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((e.points[1].coords.z - 1.5).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn push_tagged_triangle() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 1.]));
        mesh.push_tagged_triangle(Tri::new([0, 1, 2]), &name);

        match mesh.triangles_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 0); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn get_tri_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 2.]));
        mesh.triangles.push(Tri::new([0, 1, 2]));

        let tri = mesh.get_tri_view(&mesh.triangles[0]);
        assert!((tri.points[1].coords.x - 1.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((tri.points[1].coords.y - 0.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((tri.points[1].coords.z - 2.).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn push_tagged_quadrangle() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 2.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 2.]));
        mesh.push_tagged_quadrangle(Quad::new([0, 1, 2, 3]), &name);

        match mesh.quadrangles_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 0); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn get_quad_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 2.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 2.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 2.]));
        mesh.quadrangles.push(Quad::new([0, 1, 2, 3]));

        let quad = mesh.get_quad_view(&mesh.quadrangles[0]);
        assert!((quad.points[3].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((quad.points[3].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((quad.points[3].coords.z - 2.).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn push_tagged_tetrahedron() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 0.5, 1.]));
        mesh.push_tagged_tetrahedron(Tet::new([0, 1, 2, 3]), &name);

        match mesh.tetrahedra_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 0); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn get_tet_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 0.5, 1.]));
        mesh.tetrahedra.push(Tet::new([0, 1, 2, 3]));

        let tet = mesh.get_tet_view(&mesh.tetrahedra[0]);
        assert!((tet.points[3].coords.x - 0.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((tet.points[3].coords.y - 0.5).abs() < GEOMETRICAL_TOLERANCE);
        assert!((tet.points[3].coords.z - 1.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn push_tagged_hexahedron() {

        let mut mesh = Mesh3d::default();

        let name = String::from("tag");
        mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 1.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 1.]));
        mesh.push_tagged_hexahedron(Hexa::new([0, 1, 2, 3, 4, 5, 6, 7]), &name);

        match mesh.hexahedra_tags.get_registered_indexes(&name) {
            Some(indexes) => { assert_eq!(indexes[0], 0); },
            None => { assert!(false); }
        }
    }

    #[test]
    fn get_hexa_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 0.]));
        mesh.vertices.push(Pnt3d::new([0., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 1.]));
        mesh.vertices.push(Pnt3d::new([1., 1., 1.]));
        mesh.vertices.push(Pnt3d::new([0., 1., 1.]));
        mesh.hexahedra.push(Hexa::new([0, 1, 2, 3, 4, 5, 6, 7]));

        let quad = mesh.get_hexa_view(&mesh.hexahedra[0]);
        assert!((quad.points[7].coords.x - 0.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((quad.points[7].coords.y - 1.).abs() < GEOMETRICAL_TOLERANCE);
        assert!((quad.points[7].coords.z - 1.).abs() < GEOMETRICAL_TOLERANCE);
    }
}