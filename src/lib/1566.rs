// Time taken: 20:25, 20:38 -> Wrong, 20:46 -> Acc
struct Solution {}

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let mut pattern = Vec::new();
        let mut idx = 0;
        let m = m as usize;
        while idx + m - 1 < arr.len() {
            pattern.clear();
            for num in idx..(idx + m) {
                pattern.push(arr[num]);
            }
            let pattern_clone = pattern.clone();
            for _ in 1..k {
                pattern.append(&mut pattern_clone.clone());
            }
            let mut idx1 = 0;

            while idx1 < arr.len() {
                let mut left = idx1;
                let mut right = 0;
                while left < arr.len() {
                    if arr[left] == pattern[right] {
                        left += 1;
                        right += 1;
                        if right == pattern.len() {
                            return true;
                        }
                    } else {
                        break;
                    }
                }

                idx1 += 1;
            }
            idx += 1;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3),
            true
        );
        assert_eq!(
            Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2),
            true
        );
        assert_eq!(
            Solution::contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3),
            false
        );
        assert_eq!(Solution::contains_pattern(vec![1, 2, 3, 1, 2], 2, 2), false);
    }
}
