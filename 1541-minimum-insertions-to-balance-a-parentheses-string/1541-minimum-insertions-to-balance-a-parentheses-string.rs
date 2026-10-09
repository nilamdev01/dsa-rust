
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut insertions = 0;
        let mut open = 0;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut i = 0;

        while i < n {
            if chars[i] == '(' {
                open += 1;
            } else {
                if i + 1 < n && chars[i + 1] == ')' {
                    i += 1;
                } else {
                    insertions += 1;
                }

                if open > 0 {
                    open -= 1;
                } else {
                    insertions += 1;
                }
            }

            i += 1;
        }

        insertions + open * 2
    }
}
