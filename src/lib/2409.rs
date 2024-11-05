// Time taken: 23:34, 23:45, 23:49 -> Acc
struct Solution {}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let alice_arrival_date = Self::calculate_date(&arrive_alice);
        let alice_depart_date = Self::calculate_date(&leave_alice);
        let bob_arrival_date = Self::calculate_date(&arrive_bob);
        let bob_depart_date = Self::calculate_date(&leave_bob);

        if (alice_arrival_date >= bob_arrival_date && alice_arrival_date <= bob_depart_date)
            || (bob_arrival_date >= alice_arrival_date && bob_arrival_date <= alice_depart_date)
        {
            return (std::cmp::min(alice_depart_date, bob_depart_date)
                - std::cmp::max(alice_arrival_date, bob_arrival_date)
                + 1) as i32;
        }

        return 0;
    }

    pub fn calculate_date(date: &String) -> u16 {
        let mm = u16::from_str_radix(&date[0..2], 10).unwrap();
        let dd = u16::from_str_radix(&date[3..], 10).unwrap();
        let mut result = 0;
        let month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        for idx in 0..(mm - 1) {
            result += month[idx as usize];
        }

        result += dd;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            ),
            3
        );
        assert_eq!(
            Solution::count_days_together(
                "10-01".to_string(),
                "10-31".to_string(),
                "11-01".to_string(),
                "12-31".to_string()
            ),
            0
        );
    }
}
