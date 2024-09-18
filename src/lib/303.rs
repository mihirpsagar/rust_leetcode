// Time taken: 16:57, 17:01 -> Acc, 17:12 -> Optimized
struct Solution {}

impl Solution {}

struct NumArray {
    arr: Vec<i32>,
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            sum: nums.iter().fold(vec![], |mut acc, &e| {
                acc.push(e + acc.get(acc.len() - 1).unwrap_or(&0));
                acc
            }),
            arr: nums,
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.sum[right as usize] - self.sum[left as usize] + self.arr[left as usize];
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
