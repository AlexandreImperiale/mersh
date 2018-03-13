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
#[derive(Default, Serialize, Deserialize)]
pub struct TagSet {
    /// Mapping between tag, represented as String, and set of indexes.
    tag_map: HashMap<String, Vec<usize>>,
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementation.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl TagSet {
    /// Accessing potential indexes associated to a tag name.
    ///
    /// * `name` - Name of the tag,
    ///
    pub fn get_registered_indexes(&self, name: &str) -> Option<&Vec<usize>>
    {
        self.tag_map.get(name)
    }

    /// Registering a new index associated to a tag name.
    /// If the tag name was not used before, the tag name is added in the set.
    ///
    /// * `name` - Name of the tag,
    /// * `idx` - index to associate with tag.
    ///
    pub fn register(&mut self, name: &str, idx: usize)
    {
        if let Some(indexes) = self.tag_map.get_mut(name) {
            indexes.push(idx);
            return;
        }
        self.tag_map.insert(String::from(name), vec![idx]);
    }
}