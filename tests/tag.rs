extern crate mersh;

mod tag {

    use mersh::tag::*;

    #[test]
    fn get_registered_indexes() {

        let tag_set = TagSet::default();
        let indexes = tag_set.get_registered_indexes(&"tag_name_0".to_string());
        assert_eq!(indexes, None);
    }

    #[test]
    fn register() {

        let mut tag_set = TagSet::default();
        let name0 = String::from("tag_name_0");
        let name1 = String::from("tag_name_1");
        let name2 = String::from("tag_name_2");

        tag_set.register(&name0, 0);
        tag_set.register(&name0, 85);
        tag_set.register(&name1, 2);

        match tag_set.get_registered_indexes(&name0) {
            Some(indexes) => { assert_eq!(indexes[0], 0); assert_eq!(indexes[1], 85);},
            None => { assert!(false); }
        }

        match tag_set.get_registered_indexes(&name1) {
            Some(indexes) => { assert_eq!(indexes[0], 2);},
            None => { assert!(false); }
        }

        if let Some(_) = tag_set.get_registered_indexes(&name2) {
            assert!(false);
        }
    }
}