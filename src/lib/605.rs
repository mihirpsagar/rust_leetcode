// Time taken: 21:03, 21:09 -> Wrong, 21:15 -> Acc
struct Solution {}

impl Solution {
    // pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    //     let mut n = n;
    //     let mut idx = 0;

    //     while n > 0 && idx < flowerbed.len() {
    //         if flowerbed[idx] == 1 {
    //             idx += 2;
    //         } else if idx == flowerbed.len() - 1 || flowerbed[idx + 1] == 0 {
    //             n -= 1;
    //             if n == 0 {
    //                 return true;
    //             }
    //             idx += 2;
    //         } else {
    //             idx += 3;
    //         }
    //     }

    //     return n <= 0;
    // }

    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut flowerbed = flowerbed;
        let mut idx = 0;

        if flowerbed.len() == 1 && flowerbed[0] == 0 {
            n -= 1;
        }

        while n > 0 && idx < flowerbed.len() {
            if idx == 0 {
                if flowerbed[idx] == 0 && flowerbed[idx + 1] == 0 {
                    n -= 1;
                    flowerbed[idx] = 1;
                }
            } else if idx == flowerbed.len() - 1 {
                if flowerbed[idx - 1] == 0 && flowerbed[idx] == 0 {
                    n -= 1;
                    flowerbed[idx] = 1;
                }
            } else {
                if flowerbed[idx] == 0 && flowerbed[idx - 1] == 0 && flowerbed[idx + 1] == 0 {
                    n -= 1;
                    flowerbed[idx] = 1;
                }
            }
            idx += 1;
        }

        return n <= 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
        assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![0], 0), true);
    }
}
