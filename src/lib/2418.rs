// Time taken: 23:54, 23:59 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut arr = Vec::new();
        let mut result = Vec::new();

        for (idx, &val) in heights.iter().enumerate() {
            Self::binary_insert(&mut arr, idx, val);
        }

        for val in arr {
            result.push(names[val.1].clone());
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<(i32, usize)>, idx: usize, target: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid].0 < target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        arr.insert(left, (target, idx));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            ["Mary", "Emma", "John"]
        );
        assert_eq!(
            Solution::sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            ["Bob", "Alice", "Bob"]
        );
    }
}
