pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        String::from("hello world")
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
}
