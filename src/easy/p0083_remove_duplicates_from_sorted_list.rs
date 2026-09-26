//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_node_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &value in values.iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut vec_of_non_duplicaded_values: Vec<i32> = Vec::new();

        while let Some(node) = &current {
            if !vec_of_non_duplicaded_values.contains(&node.val) {
                vec_of_non_duplicaded_values.push(node.val);
            }

            current = &node.next
        }

        create_node_list(&vec_of_non_duplicaded_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;

        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = head;
            head = Some(node);
        }

        head
    }

    #[test]
    fn p0083_remove_duplicates_from_sorted_list_1() {
        let list = create_list(&[1, 1, 2]);
        let expected = create_list(&[1, 2]);

        assert_eq!(Solution::delete_duplicates(list), expected);
    }
    #[test]
    fn p0083_remove_duplicates_from_sorted_list_2() {
        let list = create_list(&[1, 1, 2, 3, 3]);
        let expected = create_list(&[1, 2, 3]);

        assert_eq!(Solution::delete_duplicates(list), expected);
    }
}
