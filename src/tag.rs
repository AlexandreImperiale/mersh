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
    /// Registering a new index associated to a tag name.
    /// If the tag name was not used before, the tag name is added in the set.
    /// Return a reference to the set of indexes associated to the tag name.
    ///
    /// # Example
    /// ```
    /// use mersh::tag::*;
    ///
    /// let mut tag_set = TagSet::default();
    ///
    /// {
    ///     let indexes = tag_set.register(&"tag_name_0".to_string(), 0);
    ///     assert!(indexes.len() == 1);
    ///     assert!(indexes[0] == 0);
    /// }
    ///
    /// {
    ///     let indexes = tag_set.register(&"tag_name_0".to_string(), 85);
    ///     assert!(indexes.len() == 2);
    ///     assert!(indexes[1] == 85);
    /// }
    ///
    /// {
    ///     let indexes = tag_set.register(&"tag_name_1".to_string(), 62);
    ///     assert!(indexes.len() == 1);
    ///     assert!(indexes[0] == 62);
    /// }
    /// ```
    pub fn register(&mut self, name: &String, idx: usize) -> &Vec<usize>
    {
        if self.tag_map.contains_key(name) {
            self.tag_map.get_mut(name).unwrap().push(idx);
        }
        else {
            self.tag_map.insert(name.clone(), vec![idx]);
        }

        self.tag_map.get(name).unwrap()
    }
}