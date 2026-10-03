impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ans = 0;

        // Left -> Right
        let mut left = 0;
        let mut right = 0;

        for &c in bytes {
            if c == b'(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                ans = ans.max(2 * right);
            } else if right > left {
                left = 0;
                right = 0;
            }
        }

        // Right -> Left
        left = 0;
        right = 0;

        for &c in bytes.iter().rev() {
            if c == b'(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                ans = ans.max(2 * left);
            } else if left > right {
                left = 0;
                right = 0;
            }
        }

        ans
    }
}