// Time taken: 23:53, 23:58 -> Acc
struct Solution {}

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut arr = Vec::new();

        for num in nums {
            if arr.len() < 4 {
                arr.push(num);
                arr.sort();
            } else {
                if num < arr[0] {
                    arr[1] = arr[0];
                    arr[0] = num;
                } else if num < arr[1] {
                    arr[1] = num;
                } else if num > arr[3] {
                    arr[2] = arr[3];
                    arr[3] = num;
                } else if num > arr[2] {
                    arr[2] = num;
                } else {
                    continue;
                }
            }
        }

        return (arr[3] * arr[2]) - (arr[1] * arr[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
            64
        );
    }
}
