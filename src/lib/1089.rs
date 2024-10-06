// Time taken: 21:31, 21:35 -> Acc
struct Solution {}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut idx = 0;
        let len = arr.len();
        while idx < len {
            if arr[idx] == 0 {
                arr.insert(idx, 0);
                idx += 2;
                arr.pop();
            } else {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, [1, 0, 0, 2, 3, 0, 0, 4]);

        arr = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
}
