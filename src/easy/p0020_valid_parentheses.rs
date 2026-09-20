use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let matching_symbol = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
        let mut opening_symbols_buffer: Vec<char> = vec![];

        // Valid answers must have a number of characters that can by divided by 2
        if s.len() % 2 != 0 {
            return false;
        }

        for character in s.chars() {
            if matches!(character, '(' | '[' | '{') {
                opening_symbols_buffer.push(character);
            } else {
                if let Some(opening_symbol) = opening_symbols_buffer.pop() {
                    if matching_symbol[&opening_symbol] != character {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        if !opening_symbols_buffer.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p0020_valid_parentheses_1() {
        let input = String::from("()");
        assert!(Solution::is_valid(input));
    }

    #[test]
    fn p0020_valid_parentheses_2() {
        let input = String::from("()[]{}");
        assert!(Solution::is_valid(input));
    }

    #[test]
    fn p0020_valid_parentheses_3() {
        let input = String::from("(]");
        assert!(!Solution::is_valid(input));
    }

    #[test]
    fn p0020_valid_parentheses_4() {
        let input = String::from("([])");
        assert!(Solution::is_valid(input));
    }

    #[test]
    fn p0020_valid_parentheses_5() {
        let input = String::from("([)]");
        assert!(!Solution::is_valid(input));
    }
}
