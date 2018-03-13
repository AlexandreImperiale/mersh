extern crate mersh;

mod edge {

    use mersh::elements::*;

    #[test]
    fn new() {
        let idx = [0, 13];
        let edge = Edge::new(idx);
        assert_eq!(edge.indexes, idx);
    }
}

mod tri {

    use mersh::elements::*;

    #[test]
    fn new() {
        let idx = [0, 13, 24];
        let tri = Tri::new(idx);
        assert_eq!(tri.indexes, idx);
    }
}

mod quad {

    use mersh::elements::*;

    #[test]
    fn new() {

        let idx = [0, 13, 53, 21];
        let quad = Quad::new(idx);
        assert_eq!(quad.indexes, idx);
    }

}

mod tet {

    use mersh::elements::*;

    #[test]
    fn new() {
        let idx = [0, 13, 34, 98];
        let tet = Tet::new(idx);
        assert_eq!(tet.indexes, idx);
    }

}

mod hexa {

    use mersh::elements::*;

    #[test]
    fn new() {
        let idx = [0, 13, 34, 98, 35, 69, 90, 43];
        let hexa = Hexa::new(idx);
        assert_eq!(hexa.indexes, idx);
    }

}