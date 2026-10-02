impl Solution {
    fn backtrack(
        s: String,
        open: i32,
        close: i32,
        n: i32,
        ans: &mut Vec<String>,
    ) {
        // Complete valid parentheses
        if open == n && close == n {
            ans.push(s);
            return;
        }

        // Add opening bracket
        if open < n {
            let mut new_s = s.clone();
            new_s.push('(');

            Self::backtrack(
                new_s,
                open + 1,
                close,
                n,
                ans,
            );
        }

        // Add closing bracket only when valid
        if close < open {
            let mut new_s = s.clone();
            new_s.push(')');

            Self::backtrack(
                new_s,
                open,
                close + 1,
                n,
                ans,
            );
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();

        Self::backtrack(
            String::new(),
            0,
            0,
            n,
            &mut ans,
        );

        ans
    }
}