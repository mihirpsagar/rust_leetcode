// Time taken: 10:45, 10:47 -> Acc
struct Solution {}

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        let mut k = k;
        let mut result = std::cmp::min(num_ones, k);
        k -= result;

        if k > 0 {
            k -= num_zeros;
        }

        if k > 0 {
            result = result - std::cmp::min(k, num_neg_ones);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }
}
