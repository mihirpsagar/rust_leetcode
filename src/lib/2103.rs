// Time taken: 18:30, 18:38 -> Acc
struct Solution {}

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut count = vec![(0, 0, 0); 10]; // (R,G,B)
        let mut idx = 0;
        let rings = rings.chars().collect::<Vec<char>>();
        let zero_ascii = '0' as u8;
        let mut result = 0;

        while idx < rings.len() {
            let pos = rings[idx + 1] as u8 - zero_ascii;

            if rings[idx] == 'R' {
                count[pos as usize].0 += 1;
            } else if rings[idx] == 'G' {
                count[pos as usize].1 += 1;
            } else {
                count[pos as usize].2 += 1;
            }

            idx += 2;
        }

        for val in count {
            if val.0 != 0 && val.1 != 0 && val.2 != 0 {
                result += 1;
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
        assert_eq!(Solution::count_points("B0B6G0R6R0R6G9".to_string()), 1);
        assert_eq!(Solution::count_points("B0R0G0R9R0B0G0".to_string()), 1);
        assert_eq!(Solution::count_points("G4".to_string()), 0);
    }
}
