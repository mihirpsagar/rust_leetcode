// Time taken: 07:46, 07:53 -> Acc
struct Solution {}

impl Solution {
    pub fn merge_arrays(mut nums1: Vec<Vec<i32>>, mut nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        nums1.sort_by(|a, b| a[0].cmp(&b[0]));
        nums2.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut left = 0;
        let mut right = 0;

        while left < nums1.len() && right < nums2.len() {
            let left_val = &nums1[left];
            let right_val = &nums2[right];

            match left_val[0].cmp(&right_val[0]) {
                std::cmp::Ordering::Equal => {
                    result.push(vec![left_val[0], left_val[1] + right_val[1]]);
                    left += 1;
                    right += 1;
                }
                std::cmp::Ordering::Less => {
                    result.push(vec![left_val[0], left_val[1]]);
                    left += 1;
                }
                std::cmp::Ordering::Greater => {
                    result.push(vec![right_val[0], right_val[1]]);
                    right += 1;
                }
            }
        }

        while left < nums1.len() {
            result.push(nums1[left].clone());
            left += 1;
        }

        while right < nums2.len() {
            result.push(nums2[right].clone());
            right += 1;
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
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3], vec![4, 5]],
                vec![vec![1, 4], vec![3, 2], vec![4, 1]]
            ),
            [[1, 6], [2, 3], [3, 2], [4, 6]]
        );
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![2, 4], vec![3, 6], vec![5, 5]],
                vec![vec![1, 3], vec![4, 3]]
            ),
            [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]
        );
    }
}
