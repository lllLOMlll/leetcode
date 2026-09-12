use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // SOLUTION #1
        // This is my first solution. Not optimal
        // I`using 2 for loop. I can do better

        // for (index1, number1) in nums.iter().enumerate() {
        //     for (index2, number2) in nums.iter().enumerate() {
        //         if number1 + number2 == target && index1 != index2 {
        //             return vec![index1 as i32, index2 as i32];
        //         }
        //     }
        // }
        // Vec::new()

        // SOLUTION #2
        // Im̀ using a hash map instead of 2 loop
        // Initially, I was inserting in the hashMap at the first line of the loop. Was working for
        // example_1 and example_2, but not for example_3
        // You need to insert in the hashMap after
        let mut hash_map_num_index: HashMap<i32, usize> = HashMap::new();
        for (index, number) in nums.iter().enumerate() {
            if hash_map_num_index.contains_key(&(target - *number)) {
                return vec![hash_map_num_index[&(target - *number)] as i32, index as i32];
            }

            hash_map_num_index.insert(*number, index);
        }
        Vec::new()
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
