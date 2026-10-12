impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums: Vec<usize> = (1..=n as usize).collect();
        let mut fact = vec![1usize; n as usize + 1];

        for i in 1..=n as usize {
            fact[i] = fact[i - 1] * i;
        }

        let mut k = k as usize - 1;
        let mut result = String::with_capacity(n as usize);

        for i in (1..=n as usize).rev() {
            let index = k / fact[i - 1];
            k %= fact[i - 1];

            result.push(char::from(b'0' + nums.remove(index) as u8));
        }

        result
    }
}