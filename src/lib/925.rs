// Time taken: 01:04, 01:07 -> Wrong, 01:12 -> Wrong, 01:20 -> Wrong, 01:21 -> Wrong, 01:22 -> Acc
struct Solution {}

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name: Vec<char> = name.chars().collect();
        let typed: Vec<char> = typed.chars().collect();

        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut last_ch = None;

        while idx2 < typed.len() {
            if idx1 < name.len() {
                if name[idx1] == typed[idx2] {
                    last_ch = Some(name[idx1]);
                    idx1 += 1;
                } else {
                    if let Some(ch) = last_ch {
                        if ch != typed[idx2] {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                if let Some(ch) = last_ch {
                    if ch != typed[idx2] {
                        return false;
                    }
                }
            }
            idx2 += 1;
        }

        if idx1 != name.len() {
            return false;
        }

        return true;

        // while idx2 < typed.len() {
        //     if name[idx1] == typed[idx2] {
        //         idx1 += 1;
        //         if idx1 == name.len() {
        //             break;
        //         }
        //     }
        //     idx2 += 1;
        // }

        // if idx1 != name.len() {
        //     return false;
        // }

        // while idx2 < typed.len() {
        //     if typed[idx2] != name[name.len() - 1] {
        //         return false;
        //     }
        //     idx2 += 1;
        // }

        // return true;
    }
}

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_string(), "aaleexa".to_string()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_string(), "aaleelx".to_string()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name(
                "zlexya".to_string(),
                "aazlllllllllllllleexxxxxxxxxxxxxxxya".to_string()
            ),
            false
        );
    }
}
