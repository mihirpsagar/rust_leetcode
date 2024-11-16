// Time taken: 11:20, 11:23 -> Wrong, 11:27 -> Acc
struct Solution {}

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut arr = [0; 10];
        let mut val = n;
        while val != 0 {
            arr[(val % 10) as usize] += 1;
            val /= 10;
        }

        val = n * 2;
        while val != 0 {
            arr[(val % 10) as usize] += 1;
            val /= 10;
        }

        val = n * 3;
        while val != 0 {
            arr[(val % 10) as usize] += 1;
            val /= 10;
        }

        if arr[0] != 0 {
            return false;
        }

        let mut idx = 1;
        while idx < arr.len() {
            if arr[idx] != 1 {
                return false;
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_fascinating(192), true);
        assert_eq!(Solution::is_fascinating(100), false);
        assert_eq!(Solution::is_fascinating(783), false);
    }
}
