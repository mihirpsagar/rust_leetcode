// Time taken: 11:56, 12:16 -> Acc
struct Solution {}

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = Vec::new();

        if turned_on == 0 {
            return vec![String::from("0:00")];
        }

        for hour_turned_on in 0..(turned_on + 1) {
            let minute_turned_on = turned_on - hour_turned_on;

            for h in 0_i32..12 {
                if h.count_ones() as i32 == hour_turned_on {
                    for m in 0_i32..60 {
                        if m.count_ones() as i32 == minute_turned_on {
                            result.push(format!("{}:{:#02}", h, m));
                        }
                    }
                }
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
        assert_eq!(Solution::read_binary_watch(0), vec!["0:00"]);
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }
}
