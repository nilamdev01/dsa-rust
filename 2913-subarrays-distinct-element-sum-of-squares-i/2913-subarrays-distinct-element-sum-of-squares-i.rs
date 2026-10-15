
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        
        for i in 0..n {
            let mut freq = [0; 101];
            let mut distinct = 0;

            for j in i..n {
                let x = nums[j] as usize;

                if freq[x] == 0 {
                    distinct += 1;
                }

                freq[x] += 1;
                ans += distinct * distinct;
            }
        }

        ans
    }
}
