// Time taken: 22:50, 23:05 -> Acc
struct Solution {}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = Vec::new();
        for _ in 0..3 {
            board.push(vec![' '; 3]);
        }

        let mut turn = 0;
        for val in moves {
            if turn == 0 {
                board[val[0] as usize][val[1] as usize] = 'X';
            } else {
                board[val[0] as usize][val[1] as usize] = 'O';
            }
            turn = (turn + 1) % 2;
        }

        let mut is_empty = false;

        for idx1 in 0..3 {
            if board[idx1][0] == board[idx1][1] && board[idx1][0] == board[idx1][2] {
                if board[idx1][0] == 'X' {
                    return String::from("A");
                } else if board[idx1][0] == 'O' {
                    return String::from("B");
                } else {
                    is_empty = true;
                }
            }
            if board[0][idx1] == board[1][idx1] && board[0][idx1] == board[2][idx1] {
                if board[0][idx1] == 'X' {
                    return String::from("A");
                } else if board[0][idx1] == 'O' {
                    return String::from("B");
                } else {
                    is_empty = true;
                }
            }
        }

        if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
            if board[0][0] == 'X' {
                return String::from("A");
            } else if board[0][0] == 'O' {
                return String::from("B");
            } else {
                is_empty = true;
            }
        }

        if board[0][2] == board[1][1] && board[0][2] == board[2][0] {
            if board[0][2] == 'X' {
                return String::from("A");
            } else if board[0][2] == 'O' {
                return String::from("B");
            } else {
                is_empty = true;
            }
        }

        if is_empty {
            return String::from("Pending");
        }

        for idx1 in 0..3 {
            for idx2 in 0..3 {
                if board[idx1][idx2] == ' ' {
                    return String::from("Pending");
                }
            }
        }

        return String::from("Draw");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![2, 0],
                vec![1, 1],
                vec![2, 1],
                vec![2, 2]
            ]),
            "A"
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![2, 0]
            ]),
            "B"
        );
        assert_eq!(
            Solution::tictactoe(vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 0],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![0, 1],
                vec![0, 2],
                vec![2, 2]
            ]),
            "Draw"
        );
    }
}
