use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//
// basis has no dependencies within the lib. name shamelessly stolen
// from one of the greatest languages of all time.
//

//
// for domain objects we stick with this map structure to support
// stable ordering when serializing. this may not always be desired
// but is good for the integration tests which commit the cache.
//
pub type Map<V> = BTreeMap<String, V>;

//
// with element units and manifests we often have several keys that
// point to the same deserialized structure. instead of repeating the
// data for each key, we maintain indexes into a `vec` per key.
//
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IndexedVec<T> {
    pub items: Vec<T>,
    pub index_map: Map<Vec<usize>>,
}

//
// element units, workflow/process/task specs can all be introspected in
// a uniform fashion.
//
pub trait ElementIntrospection {
    fn push_element_ids(&self, ids: &mut Vec<String>);

    fn element_ids(&self) -> Vec<String> {
        // TODO: with_capacity
        let mut vec: Vec<String> = Vec::new();
        self.push_element_ids(&mut vec);
        vec
    }
}

impl<T> IndexedVec<T> {
    pub fn push_for_keys(&mut self, item: T, keys: &[String]) {
        let index = self.items.len();
        self.items.push(item);

        for key in keys {
            self.index_map
                .entry(key.to_string())
                .and_modify(|value| value.push(index))
                .or_insert(vec![index]);
        }
    }

    pub fn last_item_for_key(&self, key: String) -> Option<&T> {
        self.index_map
            .get(&key)
            .and_then(|v| v.last().copied())
            .and_then(|i| self.items.get(i))
    }
}

#[cfg(test)]
mod indexed_vec_tests {
    use super::IndexedVec;

    #[test]
    fn test_push_for_keys() {
        let mut iv: IndexedVec<&str> = Default::default();

        iv.push_for_keys("bob", &["key1".to_string(), "key2".to_string()]);
        iv.push_for_keys("joe", &["key3".to_string()]);
        iv.push_for_keys("sue", &["key1".to_string(), "key3".to_string()]);

        assert_eq!(iv.items, vec!["bob", "joe", "sue"]);
        assert_eq!(iv.index_map.len(), 3);
        assert_eq!(iv.index_map["key1"], vec![0, 2]);
        assert_eq!(iv.index_map["key2"], vec![0]);
        assert_eq!(iv.index_map["key3"], vec![1, 2]);
    }

    #[test]
    fn test_last_item_for_key() {
        let mut iv: IndexedVec<&str> = Default::default();

        iv.push_for_keys("bob", &["key1".to_string(), "key2".to_string()]);
        iv.push_for_keys("joe", &["key1".to_string(), "key3".to_string()]);
        iv.push_for_keys("sue", &["key3".to_string()]);

        assert_eq!(iv.last_item_for_key("key1".to_string()), Some(&"joe"));
        assert_eq!(iv.last_item_for_key("key2".to_string()), Some(&"bob"));
        assert_eq!(iv.last_item_for_key("key3".to_string()), Some(&"sue"));
        assert_eq!(iv.last_item_for_key("hey".to_string()), None);
    }
}
