use std::collections::{HashMap, HashSet};

// Time taken: 20:54, 21:04 -> Acc
struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hash_map = HashMap::new();
        let mut hash_set = HashSet::new();
        let mut result = true;
        let s_iter = s.chars();
        let mut t_iter = t.chars();

        for s_ch in s_iter {
            let t_ch = t_iter.next().unwrap();
            match hash_map.get(&s_ch) {
                None => {
                    if hash_set.contains(&t_ch) {
                        result = false;
                        break;
                    }
                    hash_map.insert(s_ch, t_ch);
                    hash_set.insert(t_ch);
                }
                Some(&t_old_ch) => {
                    if t_old_ch != t_ch {
                        result = false;
                        break;
                    }
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_isomorphic(String::from("egg"), String::from("add")),
            true
        );
        assert_eq!(
            Solution::is_isomorphic(String::from("foo"), String::from("bar")),
            false
        );
        assert_eq!(
            Solution::is_isomorphic(String::from("paper"), String::from("title")),
            true
        );
    }
}
