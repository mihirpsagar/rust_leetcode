// Time taken: 03:51, 03:52 -> Acc
struct Solution {}

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        return vec![celsius + 273.15, (celsius * 1.80) + 32.00];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_temperature(36.50), [309.65000, 97.70000]);
        assert_eq!(
            Solution::convert_temperature(122.11),
            [395.26000, 251.79800]
        );
    }
}
