extern crate std;

use std::collections::HashMap;
use std::vec::*;
use std::string::String;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Definition of tag sets.
#[derive(Default)]
pub struct TagSet {
    /// Mapping between tag, represented as String, and set of indexes.
    tag_map: HashMap<String, Vec<usize>>,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl TagSet {
    /// Accessing potential indexes associated to a tag name.
    /// * `name` - Name of the tag,
    ///
    /// # Example
    /// ```
    /// use mersh::tag::*;
    ///
    /// let mut tag_set = TagSet::default();
    /// let indexes = tag_set.get_registered_indexes(&"tag_name_0".to_string());
    /// assert!(indexes == None);
    /// ```
    pub fn get_registered_indexes(&self, name: &String) -> Option<&Vec<usize>>
    {
        self.tag_map.get(name)
    }

    /// Registering a new index associated to a tag name.
    /// If the tag name was not used before, the tag name is added in the set.
    ///
    /// * `name` - Name of the tag,
    /// * `idx` - index to associate with tag.
    ///
    /// # Example
    /// ```
    /// use mersh::tag::*;
    ///
    /// let mut tag_set = TagSet::default();
    /// let name0 = String::from("tag_name_0");
    /// let name1 = String::from("tag_name_1");
    /// let name2 = String::from("tag_name_2");
    ///
    /// tag_set.register(&name0, 0);
    /// tag_set.register(&name0, 85);
    /// tag_set.register(&name1, 2);
    ///
    /// match tag_set.get_registered_indexes(&name0) {
    ///     Some(indexes) => { assert_eq!(indexes[0], 0); assert_eq!(indexes[1], 85);},
    ///     None => { assert!(false); }
    /// }
    ///
    /// match tag_set.get_registered_indexes(&name1) {
    ///     Some(indexes) => { assert_eq!(indexes[0], 2);},
    ///     None => { assert!(false); }
    /// }
    ///
    /// if let Some(indexes) = tag_set.get_registered_indexes(&name2) {
    ///     assert!(false);
    /// }
    /// ```
    pub fn register(&mut self, name: &String, idx: usize)
    {
        if let Some(indexes) = self.tag_map.get_mut(name) {
            indexes.push(idx);
            return;
        }
        self.tag_map.insert(name.clone(), vec![idx]);
    }
}