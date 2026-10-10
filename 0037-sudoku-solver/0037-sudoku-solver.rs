
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for digit in '1'..='9' {
                        if Self::is_valid(board, row, col, digit) {
                            board[row][col] = digit;

                            if Self::solve(board) {
                                return true;
                            }

                            // Backtrack
                            board[row][col] = '.';
                        }
                    }

                    return false;
                }
            }
        }

        true
    }

    fn is_valid(
        board: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        digit: char,
    ) -> bool {
        for i in 0..9 {
            // Check row and column
            if board[row][i] == digit || board[i][col] == digit {
                return false;
            }

            // Check 3x3 sub-box
            let box_row = 3 * (row / 3) + i / 3;
            let box_col = 3 * (col / 3) + i % 3;

            if board[box_row][box_col] == digit {
                return false;
            }
        }

        true
    }
}
