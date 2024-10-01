// Time taken: 20:04, 20:09, 20:12 -> Acc
struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        for row in image {
            let mut inv_row = Vec::new();
            let mut idx = row.len() - 1;

            while idx > 0 {
                inv_row.push(if row[idx] == 0 { 1 } else { 0 });
                idx -= 1;
            }
            inv_row.push(if row[idx] == 0 { 1 } else { 0 });
            result.push(inv_row);
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
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![[1, 0, 0], [0, 1, 0], [1, 1, 1]]
        );
        assert_eq!(
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
        );
    }
}
