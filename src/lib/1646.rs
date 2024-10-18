// Time taken: 15:59, 16:06 -> Wrong, 16:07 -> Acc
struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut arr = vec![0, 1];
        let mut idx: usize = 2;
        let n = n as usize + 1;
        while idx < n {
            if idx % 2 == 0 {
                arr.push(arr[idx / 2]);
            } else {
                arr.push(arr[idx / 2] + arr[(idx / 2) + 1]);
            }
            idx += 1;
        }

        let mut max = 0;
        for num in arr {
            if num > max {
                max = num;
            }
        }

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_maximum_generated(7), 3);
        assert_eq!(Solution::get_maximum_generated(2), 1);
        assert_eq!(Solution::get_maximum_generated(3), 2);
        assert_eq!(Solution::get_maximum_generated(0), 0);
        assert_eq!(Solution::get_maximum_generated(15), 5);
    }
}
