use std::collections::HashSet;

// Time taken: 14:02, 14:05 -> Acc
struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut set = HashSet::new();
        let mut pos = (0, 0);
        set.insert(pos);

        for ch in path.chars() {
            if ch == 'N' {
                pos.1 += 1;
            } else if ch == 'S' {
                pos.1 -= 1;
            } else if ch == 'E' {
                pos.0 += 1;
            } else {
                pos.0 -= 1;
            }

            if !set.insert(pos) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
        assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
    }
}
