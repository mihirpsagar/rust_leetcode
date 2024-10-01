// Time taken: 23:55, 00:14 -> Acc
struct Solution {}

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let alice_sum: i32 = alice_sizes.iter().sum();
        let bob_sum: i32 = bob_sizes.iter().sum();
        let mut bob_sizes = bob_sizes.clone();
        let diff = (bob_sum - alice_sum) / 2;

        bob_sizes.sort();

        for num in alice_sizes {
            if bob_sizes.binary_search(&(num + diff)).is_ok() {
                return vec![num, num + diff];
            }
        }

        return vec![-1, -1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 3]),
            vec![1, 2]
        );
        assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    }
}
