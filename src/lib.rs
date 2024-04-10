use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct AttributesCache {
    values_store: Vec<Vec<u8>>,
    attributes_store: BTreeMap<String, Vec<usize>>,
}

impl AttributesCache {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn set(&mut self, value: Vec<u8>, attributes: Vec<String>) -> Result<(), String> {
        self.values_store.push(value);
        let index = self.values_store.len() - 1;
        for a in attributes {
            if let Some(v) = self.attributes_store.get_mut(&a) {
                v.push(index);
            } else {
                self.attributes_store.insert(a, vec![index]);
            }
        }

        Ok(())
    }

    pub fn get_by_attributes(&mut self, attributes: Vec<String>) -> Option<Vec<&Vec<u8>>> {
        let mut all_attributes: Vec<&Vec<usize>> = Vec::new();
        for a in attributes {
            if let Some(v) = self.attributes_store.get(&a) {
                all_attributes.push(v);
            }
        }

        let mut result: Vec<usize> = all_attributes[0].clone();

        for v in all_attributes {
            let unique_set: BTreeSet<usize> = v.into_iter().map(|i| *i).collect();
            result = unique_set.intersection(&result.into_iter().collect()).map(|i| *i).collect();
        }

        let mut intersections = Vec::new();

        for v in result {
            intersections.push(&self.values_store[v])
        }

        if intersections.len() > 0 {
            Some(intersections)
        } else {
            None
        }
    }

    pub fn get_by_value(&mut self, value: Vec<u8>) -> Option<Vec<Vec<u8>>> {
        None
    }

    pub fn delete_by_value(&mut self, value: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    pub fn delete_by_attribute(&mut self, attribute: String) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut football_cache = AttributesCache::new();
        assert_eq!(football_cache.set("messi".as_bytes().to_vec(),
                                      vec!["player".to_string(),
                                           "argentina".to_string(),
                                           "barcelona".to_string(),
                                           "psg".to_string(),
                                           "inter-miami".to_string()]), Ok(()));

        assert_eq!(football_cache.set("dimaria".as_bytes().to_vec(),
                                      vec!["player".to_string(),
                                           "argentina".to_string(),
                                           "realmadrid".to_string(),
                                           "psg".to_string(),
                                           "manutd".to_string()]), Ok(()));

        assert_eq!(football_cache.set("vinicius".as_bytes().to_vec(),
                                      vec!["player".to_string(),
                                           "realmadrid".to_string(),
                                           "brazil".to_string()]), Ok(()));

        assert_eq!(football_cache.set("arsenal".as_bytes().to_vec(),
                                      vec!["club".to_string(),
                                           "england".to_string(),
                                           "london".to_string(),
                                           "emirates".to_string()]), Ok(()));

        assert_eq!(football_cache.set("dortmund".as_bytes().to_vec(),
                                      vec!["club".to_string(),
                                           "germany".to_string(),
                                           "borussia".to_string()]), Ok(()));

        assert_eq!(football_cache.get_by_attributes(vec!["argentina".to_string()]).is_some(), true);

        if let Some(v) = football_cache.get_by_attributes(vec!["argentina".to_string()]) {
            assert_eq!(v.len(), 2);
            assert_eq!(std::str::from_utf8(v[0]).unwrap().to_string(), "messi".to_string());
            assert_eq!(std::str::from_utf8(v[1]).unwrap().to_string(), "dimaria".to_string());
        }
    }
}
