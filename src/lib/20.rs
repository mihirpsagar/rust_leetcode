// Time taken: 22:06, 22:20

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut hash_map: HashMap<char, char> = HashMap::new();
        hash_map.insert('(', ')');
        hash_map.insert('{', '}');
        hash_map.insert('[', ']');

        let mut stack: Vec<char> = Vec::new();
        let mut result = true;

        for ch in s.chars() {
            if hash_map.contains_key(&ch) {
                stack.push(ch);
            } else {
                match stack.pop() {
                    None => {
                        result = false;
                        break;
                    }
                    Some(open_ch) => {
                        if hash_map.get(&open_ch).unwrap() != &ch {
                            result = false;
                            break;
                        }
                    }
                }
            }
        }

        if stack.len() != 0 {
            result = false;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([])".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);
    }
}
