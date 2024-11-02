// Time taken: 02:21, 02:26 -> Wrong, 02:27 -> Wrong, 02:29 -> Wrong, 02:30 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let num = num.chars().collect::<Vec<char>>();
        let mut res = None;
        let mut prev = num[0];
        let mut count = 1;
        let mut idx = 1;

        while idx < num.len() {
            if num[idx] == prev {
                count += 1;
            } else {
                if count >= 3 {
                    if let Some(val) = res {
                        if val < prev {
                            res = Some(prev);
                        }
                    } else {
                        res = Some(prev);
                    }
                }
                count = 1;
                prev = num[idx];
            }
            idx += 1;
        }

        if count >= 3 {
            if let Some(val) = res {
                if val < prev {
                    res = Some(prev);
                }
            } else {
                res = Some(prev);
            }
        }

        if let Some(val) = res {
            return vec![val; 3].iter().collect();
        }

        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_good_integer("6777133339".to_string()),
            "777"
        );
        assert_eq!(Solution::largest_good_integer("2300019".to_string()), "000");
        assert_eq!(Solution::largest_good_integer("42352338".to_string()), "");
        assert_eq!(Solution::largest_good_integer("222".to_string()), "222");
    }
}
