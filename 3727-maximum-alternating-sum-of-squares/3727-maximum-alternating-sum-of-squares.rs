
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squares: Vec<i64> = nums
            .iter()
            .map(|&x| (x as i64) * (x as i64))
            .collect();

        squares.sort_unstable();

        let n = squares.len();
        let k = (n + 1) / 2;
        let mut ans = 0i64;

        for i in 0..n {
            if i < k {
                ans += squares[n - 1 - i];
            } else {
                ans -= squares[n - 1 - i];
            }
        }

        ans
    }
}
