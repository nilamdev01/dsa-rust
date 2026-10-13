
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squares: Vec<i64> = nums
            .iter()
            .map(|&x| {
                let x = x as i64;
                x * x
            })
            .collect();

        squares.sort_unstable();

        let n = squares.len();
        let k = (n + 1) / 2;

        let positive: i64 = squares[n - k..].iter().sum();
        let negative: i64 = squares[..n - k].iter().sum();

        positive - negative
    }
}
