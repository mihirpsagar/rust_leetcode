// Time taken: 11:34, 11:42 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let mut k = k % (2 * (n - 1));
        let mut idx = 0;
        let mut forward = true;

        while k > 0 {
            k -= 1;

            if forward {
                idx += 1;
                if idx == n - 1 {
                    forward = false;
                }
            } else {
                idx -= 1;
            }
        }

        return idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_child(3, 5), 1);
        assert_eq!(Solution::number_of_child(5, 6), 2);
        assert_eq!(Solution::number_of_child(4, 2), 2);
    }
}
