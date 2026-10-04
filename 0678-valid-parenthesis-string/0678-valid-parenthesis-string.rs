impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut low = 0;
        let mut high = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    low += 1;
                    high += 1;
                }

                ')' => {
                    low -= 1;
                    high -= 1;
                }

                '*' => {
                    // '*' can be ')', '(', or empty
                    low -= 1;
                    high += 1;
                }

                _ => {}
            }

            // Too many ')' characters
            if high < 0 {
                return false;
            }

            // Minimum unmatched '(' cannot be negative
            low = low.max(0);
        }

        low == 0
    }
}