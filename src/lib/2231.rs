// Time taken: 01:08, 01:14 -> Wrong, 01:15 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_integer(mut num: i32) -> i32 {
        let mut arr = Vec::new();

        while num > 0 {
            arr.push(num % 10);
            num /= 10;
        }

        arr.reverse();

        let mut idx1 = 0;
        while idx1 < arr.len() {
            let mut idx2 = idx1 + 1;
            while idx2 < arr.len() {
                if arr[idx1] < arr[idx2] && (arr[idx1] % 2 == arr[idx2] % 2) {
                    let tmp = arr[idx1];
                    arr[idx1] = arr[idx2];
                    arr[idx2] = tmp;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        let mut result = 0;
        for num in arr {
            result *= 10;
            result += num;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_integer(1234), 3412);
        assert_eq!(Solution::largest_integer(65875), 87655);
        assert_eq!(Solution::largest_integer(247), 427);
    }
}
