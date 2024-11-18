// Time taken: 18:02, 18:05 -> Acc
struct Solution {}

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        if a >= (b + c) || b >= (a + c) || c >= (a + b) {
            return String::from("none");
        }

        if a == b && a == c {
            return String::from("equilateral");
        }

        if a == b || b == c || a == c {
            return String::from("isosceles");
        }

        return String::from("scalene");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::triangle_type(vec![3, 3, 3]), "equilateral");
        assert_eq!(Solution::triangle_type(vec![3, 4, 5]), "scalene");
    }
}
