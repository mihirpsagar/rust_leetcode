// Time taken: 22:45, 22:50, 22:55 -> Wrong, 23:07 -> Acc
struct Solution {}

impl Solution {
    pub fn min_number_of_hours(
        mut initial_energy: i32,
        mut initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut result = 0;
        let mut idx = 0;

        while idx < energy.len() {
            if initial_energy <= energy[idx] {
                result += energy[idx] - initial_energy + 1;
                initial_energy = 1;
            } else {
                initial_energy -= energy[idx];
            }
            if initial_experience <= experience[idx] {
                result += experience[idx] - initial_experience + 1;
                initial_experience = experience[idx] + 1;
            }
            initial_experience += experience[idx];
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
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        );
        assert_eq!(Solution::min_number_of_hours(2, 4, vec![1], vec![3]), 0);
        assert_eq!(
            Solution::min_number_of_hours(1, 1, vec![1, 1, 1, 1], vec![1, 1, 1, 50]),
            51
        );
    }
}
