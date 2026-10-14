
impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        if sum > 9 * num {
            return String::new();
        }

        let n = num as usize;
        let mut ans = vec![b'0'; n];
        let mut remaining = sum;

        for digit in ans.iter_mut() {
            if remaining == 0 {
                break;
            }

            let d = remaining.min(9) as u8;
            *digit += d;
            remaining -= d as i32;
        }

        String::from_utf8(ans).unwrap()
    }
}
