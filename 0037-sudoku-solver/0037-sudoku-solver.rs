
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0u16; 9];
        let mut cols = [0u16; 9];
        let mut boxes = [0u16; 9];
        let mut empty = Vec::with_capacity(81);

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];

                if ch == '.' {
                    empty.push((r, c));
                } else {
                    let bit = 1u16 << (ch as u8 - b'1');
                    rows[r] |= bit;
                    cols[c] |= bit;
                    boxes[(r / 3) * 3 + c / 3] |= bit;
                }
            }
        }

        Self::backtrack(
            board,
            &mut empty,
            0,
            &mut rows,
            &mut cols,
            &mut boxes,
        );
    }

    fn backtrack(
        board: &mut Vec<Vec<char>>,
        empty: &mut Vec<(usize, usize)>,
        pos: usize,
        rows: &mut [u16; 9],
        cols: &mut [u16; 9],
        boxes: &mut [u16; 9],
    ) -> bool {
        if pos == empty.len() {
            return true;
        }

        // Choose the cell with the fewest candidates (MRV).
        let mut best = pos;
        let mut min_count = 10;

        for i in pos..empty.len() {
            let (r, c) = empty[i];
            let b = (r / 3) * 3 + c / 3;
            let used = rows[r] | cols[c] | boxes[b];
            let candidates = 0x1FFu16 & !used;
            let count = candidates.count_ones();

            if count < min_count {
                min_count = count;
                best = i;

                if count <= 1 {
                    break;
                }
            }
        }

        if min_count == 0 {
            return false;
        }

        empty.swap(pos, best);
        let (r, c) = empty[pos];
        let b = (r / 3) * 3 + c / 3;
        let mut candidates =
            0x1FFu16 & !(rows[r] | cols[c] | boxes[b]);

        while candidates != 0 {
            let bit = candidates & candidates.wrapping_neg();
            candidates &= candidates - 1;

            let digit = bit.trailing_zeros() as u8;
            board[r][c] = (b'1' + digit) as char;

            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;

            if Self::backtrack(
                board,
                empty,
                pos + 1,
                rows,
                cols,
                boxes,
            ) {
                return true;
            }

            rows[r] ^= bit;
            cols[c] ^= bit;
            boxes[b] ^= bit;
            board[r][c] = '.';
        }

        empty.swap(pos, best);
        false
    }
}
