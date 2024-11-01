// Time taken: 21:35, 21:41 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = cost.len();
        Self::quick_sort(&mut cost, 0, len);
        let mut idx = len - 1;
        let mut count = 1;

        loop {
            if count % 3 != 0 {
                result += cost[idx];
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
            count += 1;
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

        while idx < end {
            if arr[idx] < arr[pivot] {
                let tmp = arr[left];
                arr[left] = arr[idx];
                arr[idx] = tmp;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
        assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
        assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
    }
}
