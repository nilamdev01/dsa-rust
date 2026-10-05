use std::collections::VecDeque;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut st = VecDeque::new();
        st.push_back(0);

        for c in s.chars() {
            if c == '(' {
                st.push_back(0);
            } else {
                let current = st.pop_back().unwrap();

                let score = if current == 0 {
                    1
                } else {
                    2 * current
                };

                *st.back_mut().unwrap() += score;
            }
        }

        *st.back().unwrap()
    }
}