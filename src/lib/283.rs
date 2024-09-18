// Time taken: 16:02, 16:05 -> Wrong, 16:09 -> Acc, 16:12 -> Optimized
struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut read = 0;
        let mut write = 0;

        while read < nums.len() {
            if nums[read] == 0 {
                read += 1;
                continue;
            }

            nums[write] = nums[read];
            write += 1;
            read += 1;
        }

        while write < nums.len() {
            nums[write] = 0;
            write += 1;
        }
    }

    // pub fn move_zeroes(nums: &mut Vec<i32>) {
    //     let mut curr_index = 0;
    //     for idx in 0..nums.len() {
    //         if nums[idx] != 0 {
    //             let tmp = nums[curr_index];
    //             nums[curr_index] = nums[idx];
    //             nums[idx] = tmp;
    //             curr_index += 1;
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
