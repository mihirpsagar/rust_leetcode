// Time taken: 23:08, 23:12 -> Acc
struct Solution {}

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut binary_vec = Vec::new();
        let mut n = n;

        while n > 0 {
            binary_vec.push(n % 2);
            n /= 2;
        }

        let mut max = 0;
        let mut prev = None;
        for (idx, &num) in binary_vec.iter().enumerate() {
            if num == 1 {
                if let Some(prev_idx) = prev {
                    if idx - prev_idx > max {
                        max = idx - prev_idx;
                    }
                }
                prev = Some(idx);
            }
        }

        return max as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::binary_gap(22), 2);
        assert_eq!(Solution::binary_gap(8), 0);
        assert_eq!(Solution::binary_gap(5), 2);
    }
}
