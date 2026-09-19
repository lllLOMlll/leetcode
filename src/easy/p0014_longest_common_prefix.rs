pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            "".to_string();
        }

        let mut longest_common_prefix = String::from("");
        let mut shortest_len = 0;

        for (index, word) in strs.iter().enumerate() {
            if index == 0 {
                shortest_len = word.len();
            }

            shortest_len = shortest_len.min(word.len());
        }

        for i in 0..shortest_len {
            let mut char_to_add: Option<char> = None;
            for (index, word) in strs.iter().enumerate() {
                if index == 0 {
                    char_to_add = word.chars().nth(i);
                } else {
                    if char_to_add != word.chars().nth(i) {
                        return longest_common_prefix;
                    }
                }
                if index == strs.len() - 1 {
                    if let Some(c) = char_to_add {
                        longest_common_prefix.push(c);
                    }
                }
            }
        }

        longest_common_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p0014_1() {
        let input = ["flower", "flow", "flight"].map(String::from).to_vec();
        let output = Solution::longest_common_prefix(input);

        assert_eq!(output, "fl");
    }

    #[test]
    fn p0014_2() {
        let input = ["dog", "racecar", "car"].map(String::from).to_vec();
        let output = Solution::longest_common_prefix(input);

        assert_eq!(output, "");
    }

    #[test]
    fn p00014_3() {
        let input = ["vandal", "vanilla", "van"].map(String::from).to_vec();
        let output = Solution::longest_common_prefix(input);

        assert_eq!(output, "vandal");
    }
}
