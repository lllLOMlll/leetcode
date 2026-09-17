use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_number_hash = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut sum = 0;
        let mut v_or_x_is_on = false;
        let mut l_or_c_is_on = false;
        let mut d_or_m_is_on = false;

        for c in s.chars().rev() {
            if v_or_x_is_on && (c == 'I') {
                sum -= 2;
                v_or_x_is_on = false;
            }
            if l_or_c_is_on && (c == 'X') {
                sum -= 20;
                l_or_c_is_on = false;
            }
            if d_or_m_is_on && (c == 'C') {
                sum -= 200;
                d_or_m_is_on = false;
            }

            sum += roman_number_hash[&c];

            if c == 'V' || c == 'X' {
                v_or_x_is_on = true;
            }
            if c == 'L' || c == 'C' {
                l_or_c_is_on = true;
            }
            if c == 'D' || c == 'M' {
                d_or_m_is_on = true;
            }
        }

        sum as i32
    }
}

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int_1() {
        let input = Solution::roman_to_int(String::from("III"));
        let output = 3;

        assert_eq!(input, output);
    }

    #[test]
    fn roman_to_int_2() {
        let input = Solution::roman_to_int(String::from("LVIII"));
        let output = 58;

        assert_eq!(input, output);
    }

    #[test]
    fn roman_to_int_3() {
        let input = Solution::roman_to_int(String::from("MCMXCIV"));
        let output = 1994;

        assert_eq!(input, output);
    }

    #[test]
    fn roman_to_int_4() {
        let input = Solution::roman_to_int(String::from("IV"));
        let output = 4;

        assert_eq!(input, output);
    }

    #[test]
    fn roman_to_int_5() {
        let input = Solution::roman_to_int(String::from("XL"));
        let ouput = 40;

        assert_eq!(input, ouput);
    }

    #[test]
    fn roman_to_int_6() {
        let input = Solution::roman_to_int(String::from("CD"));
        let output = 400;

        assert_eq!(input, output);
    }
}
