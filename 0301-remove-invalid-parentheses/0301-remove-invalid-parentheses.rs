use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn dfs(
            s: &[u8],
            idx: usize,
            left_rem: i32,
            right_rem: i32,
            open: i32,
            path: &mut String,
            ans: &mut Vec<String>,
            vis: &mut HashSet<String>,
        ) {
            if idx == s.len() {
                if left_rem == 0 && right_rem == 0 && open == 0 {
                    if vis.insert(path.clone()) {
                        ans.push(path.clone());
                    }
                }
                return;
            }

            let ch = s[idx] as char;

            // Letter
            if ch.is_alphabetic() {
                path.push(ch);

                dfs(
                    s,
                    idx + 1,
                    left_rem,
                    right_rem,
                    open,
                    path,
                    ans,
                    vis,
                );

                path.pop();
                return;
            }

            // Remove '('
            if ch == '(' && left_rem > 0 {
                dfs(
                    s,
                    idx + 1,
                    left_rem - 1,
                    right_rem,
                    open,
                    path,
                    ans,
                    vis,
                );
            }

            // Remove ')'
            if ch == ')' && right_rem > 0 {
                dfs(
                    s,
                    idx + 1,
                    left_rem,
                    right_rem - 1,
                    open,
                    path,
                    ans,
                    vis,
                );
            }

            // Keep '('
            if ch == '(' {
                path.push('(');

                dfs(
                    s,
                    idx + 1,
                    left_rem,
                    right_rem,
                    open + 1,
                    path,
                    ans,
                    vis,
                );

                path.pop();
            }

            // Keep ')'
            if ch == ')' && open > 0 {
                path.push(')');

                dfs(
                    s,
                    idx + 1,
                    left_rem,
                    right_rem,
                    open - 1,
                    path,
                    ans,
                    vis,
                );

                path.pop();
            }
        }

        // Find minimum removals
        let mut left_rem = 0;
        let mut right_rem = 0;

        for ch in s.chars() {
            if ch == '(' {
                left_rem += 1;
            } else if ch == ')' {
                if left_rem > 0 {
                    left_rem -= 1;
                } else {
                    right_rem += 1;
                }
            }
        }

        let mut ans = Vec::new();
        let mut vis = HashSet::new();
        let mut path = String::new();

        dfs(
            s.as_bytes(),
            0,
            left_rem,
            right_rem,
            0,
            &mut path,
            &mut ans,
            &mut vis,
        );

        ans
    }
}