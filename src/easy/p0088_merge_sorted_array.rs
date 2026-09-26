pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // SOLUTION #1 (beats 100%)
        // let poping = (n + m) - m;

        // for i in 0..poping {
        //     nums1.pop();
        // }

        // for i in 0..n {
        //     nums1.push(nums2[i as usize]);
        // }

        // nums1.sort();

        // SOLUTION #2 - A MORE IDIOMATIC SOLUTION
        nums1.truncate(m as usize);
        nums1.extend_from_slice(&nums2[..n as usize]);
        nums1.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p0088_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn p0088_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn p0088_3() {
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1])
    }
}
