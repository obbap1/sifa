use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default)]
pub struct AttributesCache {
    values_store: Vec<String>,
    attributes_store: BTreeMap<String, Vec<usize>>,
    value_to_attributes_store: BTreeMap<String, Vec<String>>,
}

impl AttributesCache {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn set(&mut self, value: String, attributes: Vec<String>) -> Result<(), String> {
        self.values_store.push(value.clone());
        let index = self.values_store.len() - 1;
        self.value_to_attributes_store.insert(value, attributes.clone());
        for a in attributes {
            if let Some(v) = self.attributes_store.get_mut(&a) {
                v.push(index);
            } else {
                self.attributes_store.insert(a, vec![index]);
            }
        }


        Ok(())
    }

    pub fn get_by_attributes(&mut self, attributes: Vec<String>) -> Option<Vec<&String>> {
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

    pub fn get_by_value(&mut self, value: String) -> Option<&Vec<String>> {
        self.value_to_attributes_store.get(&value)
    }

    pub fn delete_by_value(&mut self, value: String) -> Result<(), String> {
        // get index in values store
        let mut value_index = None;
        for (i, v) in self.values_store.iter().enumerate() {
            if *v == value {
                value_index = Some(i);
                break;
            }
        }

        if value_index.is_none() {
            return Err("value doesnt exist".to_string());
        }


        if let Some(attributes) = self.value_to_attributes_store.get(&value) {
            for attribute in attributes {
                if let Some(mut value_indexes) = self.attributes_store.get(attribute) {
                    for (i, v_index) in value_indexes.iter().enumerate() {
                        if *v_index == value_index.unwrap() {
                            value_indexes.remove(*v_index);
                        }
                    }
                }
            }
        }


        self.value_to_attributes_store.remove(&value);

        Ok(())
    }

    pub fn delete_by_attributes(&mut self, attributes: Vec<String>) -> Result<Vec<String>, String> {
        let mut deleted_values = Vec::new();
        if let Some(attribute_values) = self.get_by_attributes(attributes) {
            for v in attribute_values {
                self.delete_by_value(v.into_string()).unwrap();
                deleted_values.push(v.into_string());
            }
        }

        Ok(deleted_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut football_cache = AttributesCache::new();
        assert_eq!(football_cache.set("messi".to_string(),
                                      vec!["player".to_string(),
                                           "argentina".to_string(),
                                           "barcelona".to_string(),
                                           "psg".to_string(),
                                           "inter-miami".to_string()]), Ok(()));

        assert_eq!(football_cache.set("dimaria".to_string(),
                                      vec!["player".to_string(),
                                           "argentina".to_string(),
                                           "realmadrid".to_string(),
                                           "psg".to_string(),
                                           "manutd".to_string()]), Ok(()));

        assert_eq!(football_cache.set("vinicius".to_string(),
                                      vec!["player".to_string(),
                                           "realmadrid".to_string(),
                                           "brazil".to_string()]), Ok(()));

        assert_eq!(football_cache.set("arsenal".to_string(),
                                      vec!["club".to_string(),
                                           "england".to_string(),
                                           "london".to_string(),
                                           "emirates".to_string()]), Ok(()));

        assert_eq!(football_cache.set("dortmund".to_string(),
                                      vec!["club".to_string(),
                                           "germany".to_string(),
                                           "borussia".to_string()]), Ok(()));

        assert_eq!(football_cache.get_by_attributes(vec!["argentina".to_string()]).is_some(), true);

        if let Some(v) = football_cache.get_by_attributes(vec!["argentina".to_string()]) {
            assert_eq!(v.len(), 2);
            assert_eq!(v[0], "messi".to_string());
            assert_eq!(v[1], "dimaria".to_string());
        }
    }
}
