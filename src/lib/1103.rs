// Time taken: 21:38, 22:11
struct Solution {}

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut result = vec![0; num_people as usize];
        let mut candies = candies;
        let num_people = num_people as usize;
        let mut val = 1;
        let mut idx = 0;

        while candies > 0 {
            result[idx] += std::cmp::min(val, candies);
            candies -= val;
            val += 1;
            idx = (idx + 1) % num_people;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(7, 4), [1, 2, 3, 1]);
        assert_eq!(Solution::distribute_candies(10, 3), [5, 2, 3]);
    }
}
