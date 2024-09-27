// Time taken: 22:51, 23:08, 23:11 -> Acc
struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let adj = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut result = Vec::new();
        for _ in 0..img.len() {
            result.push(vec![0; img[0].len()]);
        }

        for x in 0..img.len() {
            for y in 0..img[0].len() {
                Self::calculate_avg(&img, &adj, x, y, &mut result);
            }
        }

        return result;
    }

    fn calculate_avg(
        img: &Vec<Vec<i32>>,
        adj: &Vec<(i32, i32)>,
        x: usize,
        y: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        let mut avg = 0;
        let mut count = 0;

        for (dx, dy) in adj {
            if (x as i32 + dx) < 0 || (y as i32 + dy) < 0 {
                continue;
            }
            let posx = (x as i32 + dx) as usize;
            let posy = (y as i32 + dy) as usize;

            if img.get(posx).is_some() {
                if img[posx].get(posy).is_some() {
                    avg += img[posx][posy];
                    count += 1;
                }
            }
        }

        result[x][y] = avg / count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        );
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![[137, 141, 137], [141, 138, 141], [137, 141, 137]]
        );
    }
}
