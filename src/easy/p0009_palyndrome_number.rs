pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let is_palyndrome = true;

        if x.is_negative() {
            return false;
        }

        let mut left: usize = 0;
        let mut right = x.to_string().len();
        let ajusted_right = right / 2;
        let x_to_string = x.to_string();

        while left < (ajusted_right) {
            let character_left = x_to_string.chars().nth(left);
            let character_right = x_to_string.chars().nth(right - 1);

            println!("Character left = {:?}", character_left);
            println!("Character right = {:?}", character_right);
            if x_to_string.chars().nth(left) != x_to_string.chars().nth(right - 1) {
                return false;
            }

            left += 1;
            right -= 1;
        }

        is_palyndrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p0009_is_palyndrome_1() {
        let result = Solution::is_palindrome(121);
        assert!(result);
    }

    #[test]
    fn p0009_is_palyndrome_2() {
        let result = Solution::is_palindrome(-121);
        assert!(!result);
    }

    #[test]
    fn p0009_is_palyndrome_3() {
        let result = Solution::is_palindrome(10);
        assert!(!result);
    }

    #[test]
    fn p0009_is_palyndrome_4() {
        let result = Solution::is_palindrome(1000030001);
        assert!(!result);
    }
}
