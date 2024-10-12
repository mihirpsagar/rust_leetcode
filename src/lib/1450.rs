// Time taken: 13:18, 13:19 -> Acc
struct Solution {}

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut result = 0;
        let mut idx = 0;

        while idx < start_time.len() {
            if start_time[idx] <= query_time && end_time[idx] >= query_time {
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
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    }
}
