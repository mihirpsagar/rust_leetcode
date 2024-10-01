// Time taken: 21:28, 21:38 -> TLE, 22:46 -> Acc
struct Solution {}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() || s.len() == 1 {
            return false;
        }

        let s = s.chars().collect::<Vec<char>>();
        let goal = goal.chars().collect::<Vec<char>>();

        let mut difference = Vec::new();
        let mut idx = 0;
        let mut duplicate = false;
        let mut set = std::collections::HashSet::new();

        while idx < s.len() {
            if s[idx] != goal[idx] {
                difference.push(idx);
                if difference.len() > 2 {
                    return false;
                }
            }
            if !set.insert(s[idx]) {
                duplicate = true;
            }
            idx += 1;
        }

        if difference.len() == 0 && duplicate {
            return true;
        } else if difference.len() == 2 {
            if s[difference[0]] == goal[difference[1]] && s[difference[1]] == goal[difference[0]] {
                return true;
            }
        }

        return false;
    }

    // pub fn buddy_strings(s: String, goal: String) -> bool {
    //     if s.len() != goal.len() || s.len() == 1 {
    //         return false;
    //     }

    //     let mut s = s.chars().collect::<Vec<char>>();
    //     let goal = goal.chars().collect::<Vec<char>>();

    //     for idx1 in 0..(s.len() - 1) {
    //         for idx2 in (idx1 + 1)..s.len() {
    //             let mut tmp = s[idx1];
    //             s[idx1] = s[idx2];
    //             s[idx2] = tmp;

    //             if s.iter().eq(goal.iter()) {
    //                 return true;
    //             }

    //             tmp = s[idx1];
    //             s[idx1] = s[idx2];
    //             s[idx2] = tmp;
    //         }
    //     }

    //     return false;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::buddy_strings("ab".to_string(), "ba".to_string()),
            true
        );
        assert_eq!(
            Solution::buddy_strings("ab".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::buddy_strings("aa".to_string(), "aa".to_string()),
            true
        );
        assert_eq!(
            Solution::buddy_strings("a".to_string(), "a".to_string()),
            false
        );
    }
}
