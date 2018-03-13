extern crate mersh;

mod edge_view3d {

    use mersh::base::*;
    use mersh::elements::*;
    use mersh::mesh::*;

    #[test]
    fn get_length() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0., 0., 0.]));
        mesh.vertices.push(Pnt3d::new([1., 0., 0.]));
        mesh.edges.push(Edge::new([0, 1]));

        let e = mesh.get_edge_view(&mesh.edges[0]);
        assert!((e.get_length() - 1.0) < 1e-10);
    }
}

mod tri_view3d {

    use mersh::base::*;
    use mersh::elements::*;
    use mersh::mesh::*;

    #[test]
    fn get_normal() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0.0, 0.0, 0.0]));
        mesh.vertices.push(Pnt3d::new([0.0, 1.0, 0.0]));
        mesh.vertices.push(Pnt3d::new([1.0, 0.0, 0.0]));
        mesh.triangles.push(Tri::new([0, 1, 2]));
        mesh.triangles.push(Tri::new([0, 2, 1]));

        let n = mesh.get_tri_view(&mesh.triangles[0]).get_normal();
        let m = mesh.get_tri_view(&mesh.triangles[1]).get_normal();

        assert!(n.coords.equals(&Coord3d{ x: 0.0, y: 0.0, z:-1.0 }, GEOMETRICAL_TOLERANCE));
        assert!(m.coords.equals(&Coord3d{ x: 0.0, y: 0.0, z: 1.0 }, GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn get_edge_view() {

        let mut mesh = Mesh3d::default();

        mesh.vertices.push(Pnt3d::new([0.0, 0.0, 0.0]));
        mesh.vertices.push(Pnt3d::new([0.0, 1.0, 0.0]));
        mesh.vertices.push(Pnt3d::new([2.0, 0.0, 0.0]));
        mesh.triangles.push(Tri::new([0, 1, 2]));

        let e = mesh.get_tri_view(&mesh.triangles[0]).get_edge_view(EdgeInTri::Edge20);

        assert!((e.get_length() - 2.0).abs() < 1e-10);

    }
}



