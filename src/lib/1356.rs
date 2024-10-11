// Time taken: 22:31, 22:35 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| {
            let c1 = a.count_ones();
            let c2 = b.count_ones();

            if c1 != c2 {
                return c1.cmp(&c2);
            } else {
                return a.cmp(&b);
            }
        });

        return arr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            [0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
