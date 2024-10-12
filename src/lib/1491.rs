// Time taken: 13:58, 14:00 -> Acc
struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min = salary[0];
        let mut max = salary[0];
        let mut sum = 0;
        let len = (salary.len() - 2) as f64;

        for num in salary {
            sum += num;
            if num < min {
                min = num;
            }
            if num > max {
                max = num;
            }
        }

        sum -= min;
        sum -= max;

        return sum as f64 / len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500.00000);
        assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000.00000);
    }
}
