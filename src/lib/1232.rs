// Time taken: 17:30, 17:49 -> Acc
struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // y = mx + c, m = (y2 - y1) / (x2 - x1), c = y1 - m(x1)
        enum Formula {
            Slope,
            X,
        }

        let mut formula = Formula::Slope;
        let mut m = 0.0;
        let mut c = 0.0;

        if coordinates.len() <= 2 {
            return true;
        }

        if coordinates[0][0] == coordinates[1][0] {
            formula = Formula::X;
        } else {
            m = (coordinates[1][1] as f64 - coordinates[0][1] as f64)
                / (coordinates[1][0] as f64 - coordinates[0][0] as f64);
            c = coordinates[0][1] as f64 - (m * coordinates[0][0] as f64);
        }

        let mut idx = 2;
        while idx < coordinates.len() {
            match formula {
                Formula::Slope => {
                    if coordinates[idx][1] as f64 != (m * coordinates[idx][0] as f64) + c {
                        return false;
                    }
                }
                Formula::X => {
                    if coordinates[idx][0] != coordinates[0][0] {
                        return false;
                    }
                }
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
        assert_eq!(
            Solution::check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7]
            ]),
            true
        );
        assert_eq!(
            Solution::check_straight_line(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![7, 7]
            ]),
            false
        );
    }
}
