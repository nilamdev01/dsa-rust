impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut ans = n;

        for (i, &num) in nums.iter().enumerate() {
            ans ^= i as i32;
            ans ^= num;
        }

        ans
    }
}