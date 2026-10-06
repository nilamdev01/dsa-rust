
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut open, mut ans) = (0, 0);

        for ch in s.bytes() {
            if ch == b'(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                ans += 1;
            }
        }

        ans + open
    }
}
