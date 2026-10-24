impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let to_day = |date: &str| -> i32 {
            let month: usize = date[0..2].parse().unwrap();
            let day: i32 = date[3..5].parse().unwrap();

            month_days[..month - 1].iter().sum::<i32>() + day
        };

        let start = to_day(&arrive_alice).max(to_day(&arrive_bob));
        let end = to_day(&leave_alice).min(to_day(&leave_bob));

        (end - start + 1).max(0)
    }
}