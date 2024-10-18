// Time taken: 22:45, 22:52 -> Acc
struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = vec![0; 2];

        for val in students {
            count[val as usize] += 1;
        }

        for val in sandwiches {
            if count[val as usize] == 0 {
                break;
            }
            count[val as usize] -= 1;
        }

        return count[0] + count[1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
            0
        );
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
