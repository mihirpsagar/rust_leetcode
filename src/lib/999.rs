// Time taken: 23:30, 23:40, 23:45 -> Acc
struct Solution {}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let mut rook_pos = (0, 0);

        let mut row = 0;
        let mut col = 0;
        'outer: while row < board.len() {
            col = 0;
            while col < board[0].len() {
                if board[row][col] == 'R' {
                    break 'outer;
                }
                col += 1;
            }
            row += 1;
        }

        rook_pos.0 = row;
        rook_pos.1 = col;

        if rook_pos.0 != 0 {
            row = rook_pos.0 - 1;
            loop {
                if board[row][rook_pos.1] == 'p' {
                    result += 1;
                    break;
                } else if board[row][rook_pos.1] == 'B' {
                    break;
                }

                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }

        if rook_pos.0 != board.len() - 1 {
            row = rook_pos.0 + 1;
            loop {
                if board[row][rook_pos.1] == 'p' {
                    result += 1;
                    break;
                } else if board[row][rook_pos.1] == 'B' {
                    break;
                }

                if row == board.len() - 1 {
                    break;
                }
                row += 1;
            }
        }

        if rook_pos.1 != 0 {
            col = rook_pos.1 - 1;
            loop {
                if board[rook_pos.0][col] == 'p' {
                    result += 1;
                    break;
                } else if board[rook_pos.0][col] == 'B' {
                    break;
                }

                if col == 0 {
                    break;
                }
                col -= 1;
            }
        }

        if rook_pos.1 != board[0].len() - 1 {
            col = rook_pos.1 + 1;
            loop {
                if board[rook_pos.0][col] == 'p' {
                    result += 1;
                    break;
                } else if board[rook_pos.0][col] == 'B' {
                    break;
                }

                if col == board[0].len() - 1 {
                    break;
                }
                col += 1;
            }
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
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        );
    }
}
