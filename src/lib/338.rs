// Time taken: 17:34, 17:49 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![0];

        for num in 1..=n {
            result.push(result[(num >> 1) as usize] + (num & 1));
        }

        return result;
    }

    // pub fn count_bits(n: i32) -> Vec<i32> {
    //     let mut result = Vec::new();

    //     for idx in 0..=n {
    //         result.push(idx.count_ones() as i32);
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
