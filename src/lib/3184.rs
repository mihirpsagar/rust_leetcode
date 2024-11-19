// Time taken: 11:44, 11:46 -> Acc
struct Solution {}

impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx1 = 0;

        while idx1 < hours.len() - 1 {
            let mut idx2 = idx1 + 1;
            while idx2 < hours.len() {
                if (hours[idx1] + hours[idx2]) % 24 == 0 {
                    result += 1;
                }

                idx2 += 1;
            }

            idx1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
