// Time taken: 18:24, 18:27 -> Acc
struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;
        let a = a as u32;
        let b = b as u32;
        let c = c as u32;

        while idx1 < arr.len() {
            let mut idx2 = idx1 + 1;
            while idx2 < arr.len() {
                if arr[idx1].abs_diff(arr[idx2]) <= a {
                    let mut idx3 = idx2 + 1;
                    while idx3 < arr.len() {
                        if arr[idx2].abs_diff(arr[idx3]) <= b && arr[idx1].abs_diff(arr[idx3]) <= c
                        {
                            result += 1;
                        }
                        idx3 += 1;
                    }
                }
                idx2 += 1;
            }
            idx1 += 1;
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
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
            4
        );
        assert_eq!(
            Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1),
            0
        );
    }
}
