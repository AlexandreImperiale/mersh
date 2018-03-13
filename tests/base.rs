extern crate mersh;

mod coord3d {

    use mersh::base::*;

    #[test]
    fn fmt() {
        let coords = Coord3d::new([1.0, 0.0, 3.0]);
        let formatted_coords = format!("{}", coords);
        assert_eq!("(1.000000, 0.000000, 3.000000)", formatted_coords);
    }

    #[test]
    fn new() {
        let coords = Coord3d::new([1.0, 0.0, 3.0]);
        assert!((coords.x - 1.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((coords.y - 0.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((coords.z - 3.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn amplify_in() {
        let mut c = Coord3d::new([1.0, 2.0, 3.0]);
        c.amplify_in(3.0);
        assert!((c.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c.y - 6.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c.z - 9.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn amplify_out() {
        let c0 = Coord3d::new([1.0, 2.0, 3.0]);
        let c1 = c0.amplify_out(3.0);
        assert!((c1.x - 3.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c1.y - 6.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c1.z - 9.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn add_in() {
        let mut c = Coord3d::new([1.0, 2.0, 3.0]);
        c.add_in(1.0, &Coord3d::new([10.0, 10.0, 10.0]));
        assert!((c.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c.y - 12.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c.z - 13.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn add_out() {
        let c0 = Coord3d::new([1.0, 2.0, 3.0]);
        let c1 = c0.add_out(1.0, &Coord3d::new([10.0, 10.0, 10.0]));
        assert!((c1.x - 11.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c1.y - 12.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c1.z - 13.0).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn mlt_add_in() {

        let a = 4.;
        let mut c0 = Coord3d::new([1.0, 3.0, 5.0]);
        let b = -2.;
        let c1 = Coord3d::new([10.0, 10.0, 10.0]);

        c0.mlt_add_in(a, b, &c1);

        assert!((c0.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c0.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!(c0.z.abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn mlt_add_out() {

        let a = 4.;
        let c0 = Coord3d::new([1.0, 3.0, 5.0]);
        let b = -2.;
        let c1 = Coord3d::new([10.0, 10.0, 10.0]);

        let c2 = c0.mlt_add_out(a, b, &c1);

        assert!((c2.x + 16.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!((c2.y + 8.0).abs() < GEOMETRICAL_TOLERANCE);
        assert!(c2.z.abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn equals() {

        let c0 = Coord3d::new([1.0, 3.4, 2.0]);
        let c1 = Coord3d::new([6.0, -13.0, -1.98]);
        let c2 = c0.clone();

        assert!(!c0.equals(&c1, GEOMETRICAL_TOLERANCE));
        assert!(c0.equals(&c2, GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn sq_norm() {
        let c0 = Coord3d::new([2.0, 2.0, 1.0]);
        assert!((c0.sq_norm() - 9.0) < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn norm() {
        let c0 = Coord3d::new([2.0, 2.0, 23.1]);
        assert!((c0.norm() - c0.sq_norm().sqrt()) < GEOMETRICAL_TOLERANCE);
    }
}

mod pnt3d {

    use mersh::base::*;

    #[test]
    fn new() {
        let p = Pnt3d::new([1.0, 0.0, 2.0]);
        assert!(p.coords.equals(&Coord3d::new([1.0, 0.0, 2.0]), GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn distance_to_0() {
        let p = Pnt3d::default();
        let q = Pnt3d::new([0.0, 1.0, 0.0]);
        assert!((p.distance_to(&q) - 1.0) < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn distance_to_1() {
        let p = Pnt3d::new([2.4, -1.9, 1.0]);
        assert!(p.distance_to(&p) < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn translate_by() {

        let p = Pnt3d::new([1.45, 3.0, 2.0]);
        let v = Vec3d::new([-0.1, 4.09, 0.98]);
        let q = p.translate_by(&v);

        assert!((q.coords.x - p.coords.x - v.coords.x).abs() < GEOMETRICAL_TOLERANCE);
        assert!((q.coords.y - p.coords.y - v.coords.y).abs() < GEOMETRICAL_TOLERANCE);
        assert!((q.coords.z - p.coords.z - v.coords.z).abs() < GEOMETRICAL_TOLERANCE);
    }

    #[test]
    fn to() {

        let p = Pnt3d::default();
        let q = Pnt3d::new([1.0, 0.0, 0.0]);

        let v0 = p.to(&q);
        let v1 = q.to(&p);

        assert!(v0.coords.equals(&Coord3d { x: 1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
        assert!(v1.coords.equals(&Coord3d { x: -1.0, y: 0.0, z: 0.0 }, GEOMETRICAL_TOLERANCE));
    }
}

mod vec3d {

    use mersh::base::*;

    #[test]
    fn new() {
        let v = Vec3d::new([1.0, 0.0, 0.0]);
        assert!(v.coords.equals(&Coord3d::new([1.0, 0.0, 0.0]), GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn cross_out_0() {

        let u = Vec3d::new([1.0, 0.0, 0.0]);
        let v = Vec3d::new([1.0, 2.0, 0.0]);
        let w = u.cross_out(&v);

        assert!(w.coords.equals(&Coord3d::new([0.0, 0.0, 2.0]), GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn cross_out_1() {

        let u = Vec3d::new([1.0, 0.0, 0.0]);
        let v = Vec3d::new([1.0, 2.0, 0.0]);
        let w = v.cross_out(&u);

        assert!(w.coords.equals(&Coord3d::new([0.0, 0.0, -2.0]), GEOMETRICAL_TOLERANCE));
    }

    #[test]
    fn normalize_out() {

        let p = Pnt3d::default();
        let q = Pnt3d::new([0.0, 3.0, 0.0]);

        let u = p.to(&q);
        let v = q.to(&p);

        let d = u.normalize_out();
        let l = v.normalize_out();

        assert!(d.coords.equals(&Coord3d::new([0.0, 1.0, 0.0]), GEOMETRICAL_TOLERANCE));
        assert!(l.coords.equals(&Coord3d::new([0.0,-1.0, 0.0]), GEOMETRICAL_TOLERANCE));
    }
}