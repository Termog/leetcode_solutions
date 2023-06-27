use std::collections::HashMap;

impl Solution {
pub fn is_anagram(mut s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: HashMap<char,i16> = HashMap::with_capacity(s.len());
        let mut s = s.chars();
        map.insert(s.next().unwrap(),1);
        for letter in s {
            if let Some(a) = map.get_mut(&letter) {
                *a += 1;
            } else {
                map.insert(letter,1);
            }
        }
        for letter in t.chars() {
            if let Some(a) = map.get_mut(&letter) {
                *a -= 1;
            } else {
                return false;
            }
        }
        match map.iter().find( |x| *x.1 != 0) {
            Some(_) => false,
            None => true,
        }
    }
}
