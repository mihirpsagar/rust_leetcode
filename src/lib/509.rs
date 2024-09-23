// Time taken: 23:45, 23:47 -> Acc
struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut f1 = 0;
        let mut f2 = 1;
        let mut idx = 1;

        while idx < n {
            let tmp = f1;
            f1 = f2;
            f2 = f1 + tmp;
            idx += 1;
        }

        return f2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
        assert_eq!(Solution::fib(4), 3);
    }
}
