//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let response: Vec<i32> = vec![];
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn empty_tree_should_return_empty_vector() {
        let root = None;

        let result = Solution::inorder_traversal(root);

        assert_eq!(result, vec![]);
    }

    #[test]
    fn single_node_should_return_single_value() {
        let root = Some(node(1));

        let result = Solution::inorder_traversal(root);

        assert_eq!(result, vec![1]);
    }

    #[test]
    fn leetcode_example_1_should_return_correct_inorder() {
        // Tree:
        //
        //     1
        //      \
        //       2
        //      /
        //     3
        //
        // Inorder: 1, 3, 2

        let root = node(1);
        let right = node(2);
        let right_left = node(3);

        right.borrow_mut().left = Some(right_left);
        root.borrow_mut().right = Some(right);

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![1, 3, 2]);
    }

    #[test]
    fn balanced_tree_should_return_correct_inorder() {
        // Tree:
        //
        //         4
        //       /   \
        //      2     6
        //     / \   / \
        //    1   3 5   7
        //
        // Inorder: 1, 2, 3, 4, 5, 6, 7

        let root = node(4);
        let node_2 = node(2);
        let node_6 = node(6);

        node_2.borrow_mut().left = Some(node(1));
        node_2.borrow_mut().right = Some(node(3));

        node_6.borrow_mut().left = Some(node(5));
        node_6.borrow_mut().right = Some(node(7));

        root.borrow_mut().left = Some(node_2);
        root.borrow_mut().right = Some(node_6);

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn left_skewed_tree_should_return_bottom_to_top() {
        // Tree:
        //
        //       4
        //      /
        //     3
        //    /
        //   2
        //  /
        // 1
        //
        // Inorder: 1, 2, 3, 4

        let root = node(4);
        let node_3 = node(3);
        let node_2 = node(2);
        let node_1 = node(1);

        node_2.borrow_mut().left = Some(node_1);
        node_3.borrow_mut().left = Some(node_2);
        root.borrow_mut().left = Some(node_3);

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn right_skewed_tree_should_return_top_to_bottom() {
        // Tree:
        //
        // 1
        //  \
        //   2
        //    \
        //     3
        //      \
        //       4
        //
        // Inorder: 1, 2, 3, 4

        let root = node(1);
        let node_2 = node(2);
        let node_3 = node(3);
        let node_4 = node(4);

        node_3.borrow_mut().right = Some(node_4);
        node_2.borrow_mut().right = Some(node_3);
        root.borrow_mut().right = Some(node_2);

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn tree_with_negative_values_should_return_correct_inorder() {
        // Tree:
        //
        //       0
        //      / \
        //    -10  10
        //      \
        //      -5
        //
        // Inorder: -10, -5, 0, 10

        let root = node(0);
        let left = node(-10);

        left.borrow_mut().right = Some(node(-5));

        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(node(10));

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![-10, -5, 0, 10]);
    }

    #[test]
    fn leetcode_example_2_should_return_correct_inorder() {
        // Tree:
        //
        //           1
        //         /   \
        //        2     3
        //       / \     \
        //      4   5     8
        //         / \   /
        //        6   7 9
        //
        // Inorder: 4, 2, 6, 5, 7, 1, 3, 9, 8

        let root = node(1);

        let node_2 = node(2);
        let node_3 = node(3);
        let node_5 = node(5);
        let node_8 = node(8);

        node_2.borrow_mut().left = Some(node(4));

        node_5.borrow_mut().left = Some(node(6));
        node_5.borrow_mut().right = Some(node(7));
        node_2.borrow_mut().right = Some(node_5);

        node_8.borrow_mut().left = Some(node(9));
        node_3.borrow_mut().right = Some(node_8);

        root.borrow_mut().left = Some(node_2);
        root.borrow_mut().right = Some(node_3);

        let result = Solution::inorder_traversal(Some(root));

        assert_eq!(result, vec![4, 2, 6, 5, 7, 1, 3, 9, 8]);
    }
}
