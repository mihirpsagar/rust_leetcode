// Time taken: 00:39, 00:42 -> Acc
struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n < 3 {
            return 1;
        }

        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;
        let mut iter = 3;

        while iter <= n {
            let tmp = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = tmp;

            iter += 1;
        }

        return t2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
