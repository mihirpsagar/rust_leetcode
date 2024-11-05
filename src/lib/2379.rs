// Time taken: 22:38, 22:44 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut black_count = 0;
        let k = k as usize;
        let mut result = k;
        let mut idx = 0;
        let blocks = blocks.chars().collect::<Vec<char>>();

        while idx < k {
            if blocks[idx] == 'B' {
                black_count += 1;
            }
            idx += 1;
        }

        result = k - black_count;

        while idx < blocks.len() {
            if blocks[idx - k] == 'B' {
                black_count -= 1;
            }
            if blocks[idx] == 'B' {
                black_count += 1;
            }

            if (k - black_count) < result {
                result = k - black_count;
            }
            idx += 1;
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }
}
