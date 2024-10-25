use std::collections::HashMap;

// Time taken: 20:05, 20:14 -> Acc
struct Solution {}

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut prod = vec![0; 251];
        let mut map = HashMap::new();
        let mut result = 0;

        for num in 1..=n {
            prod[num as usize] = num * num;
            map.insert(num * num, num);
        }

        let mut num1 = 1;
        while num1 < n {
            let mut num2 = num1 + 1;
            while num2 < n {
                if map.contains_key(&(prod[num1 as usize] + prod[num2 as usize])) {
                    result += 2;
                }
                num2 += 1;
            }
            num1 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_triples(5), 2);
        assert_eq!(Solution::count_triples(10), 4);
    }
}
