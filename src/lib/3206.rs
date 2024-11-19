// Time taken: 12:04, 12:09 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = colors.len();
        let mut idx = 0;

        while idx < len {
            let left = if idx == 0 { len - 1 } else { idx - 1 };
            let right = (idx + 1) % len;

            if colors[left] == colors[right] && colors[left] != colors[idx] {
                result += 1;
            }

            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]),
            3
        );
    }
}
