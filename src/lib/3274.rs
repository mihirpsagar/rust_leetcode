// Time taken: 13:31, 13:37 -> Acc
struct Solution {}

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let c1 = coordinate1.chars().collect::<Vec<char>>();
        let c2 = coordinate2.chars().collect::<Vec<char>>();
        let a_ascii = 'a' as u8;
        let zero_ascii = '0' as u8;
        let mut c1_color = 'B';
        let mut c2_color = 'B';

        if (c1[0] as u8 - a_ascii) % 2 == 0 {
            if (c1[1] as u8 - zero_ascii) % 2 == 0 {
                c1_color = 'W';
            }
        } else {
            if (c1[1] as u8 - zero_ascii) % 2 != 0 {
                c1_color = 'W';
            }
        }

        if (c2[0] as u8 - a_ascii) % 2 == 0 {
            if (c2[1] as u8 - zero_ascii) % 2 == 0 {
                c2_color = 'W';
            }
        } else {
            if (c2[1] as u8 - zero_ascii) % 2 != 0 {
                c2_color = 'W';
            }
        }

        return c1_color == c2_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "c3".to_string()),
            true
        );
        assert_eq!(
            Solution::check_two_chessboards("a1".to_string(), "h3".to_string()),
            false
        );
    }
}
