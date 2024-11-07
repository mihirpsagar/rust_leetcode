// Time taken: 22:57, 23:06 -> Wrong, 23:10 -> Acc
struct Solution {}

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut idx = 0;
        let mut curr = 0;
        let mut found = false;

        while idx < forts.len() {
            if forts[idx] == 0 {
                if found {
                    curr += 1;
                }
            } else if forts[idx] == -1 {
                if curr > result {
                    result = curr;
                }
                curr = 0;
                found = false;
            } else {
                found = true;
                curr = 0;
            }

            idx += 1;
        }

        idx = forts.len() - 1;
        found = false;

        loop {
            if forts[idx] == 0 {
                if found {
                    curr += 1;
                }
            } else if forts[idx] == -1 {
                if curr > result {
                    result = curr;
                }
                curr = 0;
                found = false;
            } else {
                found = true;
                curr = 0;
            }

            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
        assert_eq!(Solution::capture_forts(vec![0, 0, 1, -1]), 0);
        assert_eq!(
            Solution::capture_forts(vec![0, 0, -1, 1, -1, 1, -1, 1, 0]),
            0
        );
    }
}
