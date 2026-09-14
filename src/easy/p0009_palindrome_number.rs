pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // SOLUTION #1
        //
        // if x.is_negative() {
        //     return false;
        // }
        //
        // let mut left: usize = 0;
        // let mut right = x.to_string().len();
        // let x_to_string = x.to_string();
        //
        // while left < (right) {
        //     if x_to_string.chars().nth(left) != x_to_string.chars().nth(right - 1) {
        //         return false;
        //     }
        //
        //     left += 1;
        //     right -= 1;
        // }
        //
        // true

        //***********************************
        //SOLUTION #2
        // if x.is_negative() {
        //     return false;
        // }
        //
        // let mut digits: Vec<i32> = Vec::new();
        // let mut x_copy = x;
        // let mut last_digit: i32;
        //
        // while (x_copy) != 0 {
        //    last_digit = x_copy % 10;
        //    digits.push(last_digit);
        //
        //     x_copy /= 10;
        // }
        //
        // let mut number = 0;
        //
        // for &digit in &digits {
        //     number = number * 10 + digit;
        // }
        //
        // if x != number {
        //     return false;
        // };
        //
        // true

        //************************************
        //SOLUTION #3
        // Here, I try avoiding a second loop with for &digit in &digits. It is useless
        // I can do that directly in the while loop
        if x.is_negative() {
            return false;
        }

        let mut x_copy = x;
        let mut last_digit: i32;
        let mut number = 0;

        while (x_copy) != 0 {
            last_digit = x_copy % 10;

            number = number * 10 + last_digit;

            x_copy /= 10;
        }

        // for &digit in &digits {
        //     number = number * 10 + digit;
        // }

        if x != number {
            return false;
        };

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p0009_is_palindrome_1() {
        let result = Solution::is_palindrome(121);
        assert!(result);
    }

    #[test]
    fn p0009_is_palindrome_2() {
        let result = Solution::is_palindrome(-121);
        assert!(!result);
    }

    #[test]
    fn p0009_is_palindrome_3() {
        let result = Solution::is_palindrome(10);
        assert!(!result);
    }

    #[test]
    fn p0009_is_palindrome_4() {
        let result = Solution::is_palindrome(1000030001);
        assert!(!result);
    }
}
