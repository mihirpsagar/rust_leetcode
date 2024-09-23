// Time taken: 00:28, 00:36 -> Wrong, 00:50 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<char>>();

        if k == 1 {
            return s.iter().collect();
        }

        if s.len() <= k as usize {
            s.reverse();
            return s.iter().collect();
        }

        let mut p1: usize = 0;
        let mut p2: usize = (k - 1) as usize;
        while p2 < s.len() {
            s[p1..=p2].reverse();
            p1 += (2 * k) as usize;
            p2 += (2 * k) as usize;

            if p1 < s.len() && p2 >= s.len() {
                s[p1..].reverse();
            }
        }

        return s.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_str("abcdefg".to_string(), 2), "bacdfeg");
        assert_eq!(Solution::reverse_str("abcd".to_string(), 2), "bacd");
        assert_eq!(Solution::reverse_str("hyzqyljrnigxvdtneasepfahmtyhlohwxmkqcdfehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqlimjkfnqcqnajmebeddqsgl".to_string(), 39),
         "fdcqkmxwholhytmhafpesaentdvxginrjlyqzyhehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqllgsqddebemjanqcqnfkjmi");
    }
}
