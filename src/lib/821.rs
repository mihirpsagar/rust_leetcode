// Time taken: 02:01, 02:10, 02:14 -> Wrong, 02:17 -> Acc
struct Solution {}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut result = vec![i32::MAX; s.len()];
        let s = s.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();
        let mut last_pos = None;

        for (idx, &ch) in s.iter().enumerate() {
            if ch != c {
                stack.push(idx);
                if let Some(last_pos) = last_pos {
                    let diff = (idx - last_pos) as i32;
                    if diff < result[idx] {
                        result[idx] = diff;
                    }
                }
            } else {
                result[idx] = 0;
                last_pos = Some(idx);

                while let Some(prev_idx) = stack.pop() {
                    let diff = (idx - prev_idx) as i32;
                    if diff < result[prev_idx] {
                        result[prev_idx] = diff;
                    }
                }
            }
        }

        while let Some(prev_idx) = stack.pop() {
            let diff = last_pos.unwrap().abs_diff(prev_idx) as i32;
            if diff < result[prev_idx] {
                result[prev_idx] = diff;
            }
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
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            vec![3, 2, 1, 0]
        );
        assert_eq!(
            Solution::shortest_to_char("aaba".to_string(), 'b'),
            vec![2, 1, 0, 1]
        );
    }
}
