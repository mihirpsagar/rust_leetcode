// Time taken: 23:25, 23:28 -> Acc
struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];

        for val in encoded {
            result.push(result[result.len() - 1] ^ val);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
