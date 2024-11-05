// Time taken: 12:55, 12:56 -> Wrong, 12:58 -> Acc
struct Solution {}

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut arr = nums;

        while arr.len() > 1 {
            let mut idx = 0;
            let mut count = 0;
            let mut new_arr = Vec::new();
            while idx < arr.len() {
                if count % 2 == 0 {
                    new_arr.push(std::cmp::min(arr[idx], arr[idx + 1]));
                } else {
                    new_arr.push(std::cmp::max(arr[idx], arr[idx + 1]));
                }
                count += 1;
                idx += 2;
            }
            arr = new_arr;
        }

        return arr[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
        assert_eq!(Solution::min_max_game(vec![3]), 3);
        assert_eq!(Solution::min_max_game(vec![70, 38, 21, 22]), 22);
    }
}
