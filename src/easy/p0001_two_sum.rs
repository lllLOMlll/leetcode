pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for (index_inner_loop, number1) in nums.iter().enumerate() {
            for (index_outer_loop, number2) in nums.iter().enumerate() {
                if number1 + number2 == target && index_inner_loop != index_outer_loop {
                    let index_inner = index_inner_loop as i32;
                    let index_outer = index_outer_loop as i32;
                    result.push(index_inner);
                    result.push(index_outer);
                    return result;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);

        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let result = Solution::two_sum(vec![3, 3], 6);

        assert_eq!(result, vec![0, 1]);
    }
}
