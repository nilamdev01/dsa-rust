impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }

        let max = 10_i64.pow(n as u32) - 1;
        let min = 10_i64.pow((n - 1) as u32);

        for left in (min..=max).rev() {
            let mut x = left;
            let mut palindrome = left;

            // Create palindrome: abc -> abccba
            while x > 0 {
                palindrome = palindrome * 10 + x % 10;
                x /= 10;
            }

            // Check if palindrome = a * b
            let mut factor = max;

            while factor * factor >= palindrome {
                if palindrome % factor == 0 {
                    let other = palindrome / factor;

                    if other >= min && other <= max {
                        return (palindrome % 1337) as i32;
                    }
                }

                factor -= 1;
            }
        }

        0
    }
}