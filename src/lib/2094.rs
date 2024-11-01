use std::collections::HashSet;

// Time taken: 11:49, 12:08, 12:14 -> Acc, 12:25 -> Optimized
struct Solution {}

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 10];
        let mut result = Vec::new();

        for digit in digits {
            count[digit as usize] += 1;
        }

        'outer: for num in 100..1000 {
            if num % 2 != 0 {
                continue 'outer;
            }

            let mut required_count = vec![0; 10];
            let mut val = num;
            while val > 0 {
                required_count[(val % 10) as usize] += 1;
                val /= 10;
            }

            for idx in 0..10 {
                if required_count[idx] > count[idx] {
                    continue 'outer;
                }
            }

            result.push(num);
        }

        return result;
    }

    // pub fn find_even_numbers(mut digits: Vec<i32>) -> Vec<i32> {
    //     let mut result_set = HashSet::new();
    //     let len = digits.len();

    //     Self::quick_sort(&mut digits, 0, len);

    //     let mut idx1 = 0;
    //     'outer: while idx1 < len {
    //         if digits[idx1] == 0 {
    //             idx1 += 1;
    //             continue 'outer;
    //         }

    //         let mut idx2 = 0;

    //         'inner1: while idx2 < len {
    //             if idx2 == idx1 {
    //                 idx2 += 1;
    //                 continue 'inner1;
    //             }

    //             let mut idx3 = 0;
    //             'inner2: while idx3 < len {
    //                 if idx3 == idx1 || idx3 == idx2 {
    //                     idx3 += 1;
    //                     continue 'inner2;
    //                 }

    //                 if digits[idx3] % 2 != 0 {
    //                     idx3 += 1;
    //                     continue 'inner2;
    //                 }

    //                 result_set.insert((digits[idx1] * 100) + (digits[idx2] * 10) + digits[idx3]);
    //                 idx3 += 1;
    //             }
    //             idx2 += 1;
    //         }
    //         idx1 += 1;
    //     }

    //     let mut result = result_set.into_iter().collect::<Vec<i32>>();
    //     let len = result.len();
    //     Self::quick_sort(&mut result, 0, len);

    //     return result;
    // }

    // pub fn quick_sort(mut arr: &mut Vec<i32>, start: usize, end: usize) {
    //     if start >= end {
    //         return;
    //     }

    //     let mut left = start;
    //     let pivot = end - 1;
    //     let mut idx = start;

    //     while idx < pivot {
    //         if arr[idx] < arr[pivot] {
    //             let tmp = arr[left];
    //             arr[left] = arr[idx];
    //             arr[idx] = tmp;
    //             left += 1;
    //         }
    //         idx += 1;
    //     }

    //     let tmp = arr[left];
    //     arr[left] = arr[pivot];
    //     arr[pivot] = tmp;

    //     Self::quick_sort(&mut arr, start, left);
    //     Self::quick_sort(&mut arr, left + 1, end);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_even_numbers(vec![2, 1, 3, 0]),
            [102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
        assert_eq!(
            Solution::find_even_numbers(vec![2, 2, 8, 8, 2]),
            [222, 228, 282, 288, 822, 828, 882]
        );
        assert_eq!(Solution::find_even_numbers(vec![3, 7, 5]), []);
    }
}
