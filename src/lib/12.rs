// Time taken: 15:36, 16:08 -> Acc
struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = Vec::new();
        let mut val = num;
        let roman = vec![
            ('M', 1000),
            ('D', 500),
            ('C', 100),
            ('L', 50),
            ('X', 10),
            ('V', 5),
            ('I', 1),
        ];
        let mut idx = 0;

        while val > 0 && idx < roman.len() {
            if roman[idx].1 <= val {
                result.push(roman[idx].0);
                val -= roman[idx].1;
            } else {
                idx += 1;
            }
        }

        let mut result_str = result.iter().collect::<String>();
        result_str = result_str.replace("DCCCC", "CM");
        result_str = result_str.replace("CCCC", "CD");
        result_str = result_str.replace("LXXXX", "XC");
        result_str = result_str.replace("XXXX", "XL");
        result_str = result_str.replace("VIIII", "IX");
        result_str = result_str.replace("IIII", "IV");

        return result_str;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
