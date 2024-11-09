// Time taken: 08:05, 08:13 -> Wrong, 08:21, 08:29 -> Acc
struct Solution {}

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let val = n - 1;
        let val2 = (time - 1) / val;

        if val2 % 2 == 0 {
            if time % val == 0 {
                return n;
            } else {
                return (time % val) + 1;
            }
        } else {
            if time % val == 0 {
                return 1;
            } else {
                return n - (time % val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::pass_the_pillow(4, 5), 2);
        assert_eq!(Solution::pass_the_pillow(3, 2), 3);
        assert_eq!(Solution::pass_the_pillow(9, 4), 5);
    }
}
