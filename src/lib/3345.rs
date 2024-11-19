// Time taken: 16:20, 16:23 -> Acc
struct Solution {}

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let mut val = n;
        while !Self::is_valid(val, t) {
            val += 1;
        }

        return val;
    }

    pub fn is_valid(mut val: i32, t: i32) -> bool {
        let mut product = 1;
        while val > 0 {
            product = product * (val % 10);
            val /= 10;
        }
        return product % t == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_number(10, 2), 10);
        assert_eq!(Solution::smallest_number(15, 3), 16);
    }
}
