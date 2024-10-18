// Time taken: 22:13, 22:17 -> Acc
struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = Vec::new();
        let mut idx = 0;
        let command = command.chars().collect::<Vec<char>>();

        while idx < command.len() {
            if command[idx] == 'G' {
                result.push(command[idx]);
                idx += 1;
            } else if command[idx + 1] == ')' {
                result.push('o');
                idx += 2;
            } else {
                result.push('a');
                result.push('l');
                idx += 4;
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::interpret("G()(al)".to_string()), "Goal");
        assert_eq!(Solution::interpret("G()()()()(al)".to_string()), "Gooooal");
        assert_eq!(
            Solution::interpret("(al)G(al)()()G".to_string()),
            "alGalooG"
        );
    }
}
