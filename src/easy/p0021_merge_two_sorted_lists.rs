// Definition for singly-linked list.
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

pub struct Solution;

fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &value in values.iter().rev() {
        let mut node = Box::new(ListNode::new(value));
        node.next = head;
        head = Some(node);
    }

    head
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // SOLUTION #1 - REALLY INEFFICIENT
        // let mut current_list1 = &list1;
        // let mut current_list2 = &list2;

        // let mut vec: Vec<i32> = Vec::new();

        // while let Some(node) = current_list1 {
        //     println!("Valeur du nœud : {}", node.val);
        //     vec.push(node.val);
        //     current_list1 = &node.next;
        // }

        // while let Some(node) = current_list2 {
        //     vec.push(node.val);
        //     current_list2 = &node.next;
        // }

        // vec.sort_unstable();

        // return create_list(&vec);

        // SOLUTION #2
        let mut current1 = &list1;
        let mut current2 = &list2;
        let mut vec_of_ordered_values: Vec<i32> = Vec::new();

        // The while loop will end when one of the 2 nodes is empty -> I'll need 2 others loop to complete the work
        while let (Some(node1), Some(node2)) = (current1, current2) {
            let mininum_value = node1.val.min(node2.val);

            vec_of_ordered_values.push(mininum_value);

            if mininum_value == node1.val {
                current1 = &node1.next;
            } else {
                current2 = &node2.next;
            }
        }

        // If there is any node left in node1, add those to the vec
        while let Some(node1) = current1 {
            vec_of_ordered_values.push(node1.val);
            current1 = &node1.next;
        }

        // If there is any node left in node2, add those to the vec
        while let Some(node2) = current2 {
            vec_of_ordered_values.push(node2.val);
            current2 = &node2.next;
        }

        return create_list(&vec_of_ordered_values);
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
    fn p0021_merge_two_sorted_lists_1() {
        let list1 = create_list(&[1, 2, 4]);
        let list2 = create_list(&[1, 3, 4]);
        let expected = create_list(&[1, 1, 2, 3, 4, 4]);

        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn p0021_merge_two_sorted_lists_2() {
        let list1 = create_list(&[]);
        let list2 = create_list(&[]);
        let expected = create_list(&[]);

        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }

    #[test]
    fn p0021_merge_two_sorted_lists_3() {
        let list1 = create_list(&[]);
        let list2 = create_list(&[0]);
        let expected = create_list(&[0]);

        assert_eq!(Solution::merge_two_lists(list1, list2), expected);
    }
}
