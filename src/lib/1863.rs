// Time taken: 17:15, 17:32 -> Acc
struct Solution {}

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut permutation = vec![false; nums.len()];

        loop {
            Self::increment(&mut permutation);
            sum += Self::xor(&nums, &permutation);

            if Self::last_permutation(&permutation) {
                break;
            }
        }

        return sum;
    }

    pub fn last_permutation(arr: &Vec<bool>) -> bool {
        for val in arr {
            if !val {
                return false;
            }
        }
        return true;
    }

    pub fn xor(arr: &Vec<i32>, permutation: &Vec<bool>) -> i32 {
        let mut result = 0;
        let mut idx = 0;
        while idx < arr.len() {
            if permutation[idx] {
                result = result ^ arr[idx];
            }
            idx += 1;
        }
        return result;
    }

    pub fn increment(arr: &mut Vec<bool>) {
        let mut carry = true;
        let mut idx = 0;
        while idx < arr.len() && carry {
            let tmp = arr[idx];
            arr[idx] = carry ^ tmp;
            carry = tmp & carry;
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
