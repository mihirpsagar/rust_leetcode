// Time taken: 01:29, 01:39, 01:43 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        // area = 0.5(x1(y2-y3) + x2(y3-y1) + x3(y1-y2))
        let mut result = f64::MIN;
        let len = points.len();

        for idx1 in 0..len {
            for idx2 in (idx1 + 1)..len {
                for idx3 in (idx2 + 1)..len {
                    let p1 = &points[idx1];
                    let p2 = &points[idx2];
                    let p3 = &points[idx3];

                    let area = 0.5
                        * (p1[0] * (p2[1] - p3[1])
                            + p2[0] * (p3[1] - p1[1])
                            + p3[0] * (p1[1] - p2[1])) as f64;

                    if area.abs() > result {
                        result = area.abs();
                    }
                }
            }
        }

        // points
        //     .iter()
        //     .zip(points.iter().skip(1))
        //     .zip(points.iter().skip(2))
        //     .map(|((&p1, &p2), &p3)| {

        //     });

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        );
        assert_eq!(
            Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1],]),
            0.5
        );
    }
}
