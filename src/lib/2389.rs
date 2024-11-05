// Time taken: 23:08, 23:15 -> Wrong, 23:18 -> Acc
struct Solution {}

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let len = nums.len();
        let mut nums = nums;

        Self::quick_sort(&mut nums, 0, len);

        println!("{:?}", nums);

        for num in queries {
            result.push(Self::get_max(&nums, num));
        }

        return result;
    }

    pub fn quick_sort(mut arr: &mut Vec<i32>, start: usize, end: usize) {
        if start >= end {
            return;
        }

        let mut left = start;
        let mut idx = start;
        let pivot = end - 1;

        while idx < end - 1 {
            if arr[idx] < arr[pivot] {
                let tmp = arr[idx];
                arr[idx] = arr[left];
                arr[left] = tmp;
                left += 1;
            }
            idx += 1;
        }

        let tmp = arr[pivot];
        arr[pivot] = arr[left];
        arr[left] = tmp;

        Self::quick_sort(&mut arr, start, left);
        Self::quick_sort(&mut arr, left + 1, end);
    }

    pub fn get_max(arr: &Vec<i32>, num: i32) -> i32 {
        let mut result = 0;
        let mut sum = 0;

        for &val in arr {
            sum += val;
            if sum <= num {
                result += 1;
            } else {
                break;
            }
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
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            [2, 3, 4]
        );
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), [0]);
        assert_eq!(
            Solution::answer_queries(
                vec![736411, 184882, 914641, 37925, 214915],
                vec![331244, 273144, 118983, 118252, 305688, 718089, 665450]
            ),
            [2, 2, 1, 1, 2, 3, 3]
        );
    }
}
