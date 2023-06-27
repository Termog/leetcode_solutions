use std::collections::{BTreeMap,HashMap};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map:HashMap<BTreeMap<char,u8>,Vec<String>> = HashMap::with_capacity(strs.len());
        for string in strs {
            let mut tree: BTreeMap<char,u8> = BTreeMap::new();
            for char in string.chars() {
                match tree.get_mut(&char) {
                    Some(num) => {
                        *num += 1;
                    }
                    None => {
                        tree.insert(char,1);
                    }
                }
            }
            match map.get_mut(&tree) {
                Some(vec) => {
                    vec.push(string);
                } 
                None => {
                    map.insert(tree,vec![string]);
                }
            }
        }
        map.into_iter().map(|x| x.1).collect()
    }
}
