// Time taken: 22:18, 22:35 -> Acc, 22:40 -> Optimized
struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];

        for i in 1..(num_rows as usize) {
            let prev_vec = &result[i - 1];
            let mut next_vec = vec![1];
            for j in 1..prev_vec.len() {
                next_vec.push(prev_vec[j] + prev_vec[j - 1]);
            }
            next_vec.push(1);
            result.push(next_vec);
        }

        return result;
    }

    // pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    //     let mut result: Vec<Vec<i32>> = Vec::new();
    //     result.push(vec![1]);

    //     for i in 2..=num_rows as usize {
    //         let mut next_vec: Vec<i32> = vec![1];
    //         let mut ptr1 = 0;
    //         let mut ptr2 = 1;
    //         for _ in 0..(i - 2) as usize {
    //             next_vec.push(result[i - 2][ptr1] + result[i - 2][ptr2]);
    //             ptr1 += 1;
    //             ptr2 += 1;
    //         }
    //         next_vec.push(1);
    //         result.push(next_vec);
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(Solution::generate(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
