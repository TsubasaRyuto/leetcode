fn main() {
    println!("Hello, world!");
}

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

struct Solution {}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none()
            || head
                .as_ref()
                .map(|node| node.next.is_none())
                .unwrap_or(true)
        {
            return head;
        }

        let mut current = head.as_mut();

        while let Some(node) = current.take() {
            if let Some(next_node) = node.next.as_mut() {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                    current = Some(node);
                    continue;
                }
            }
            current = node.next.as_mut();
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Array to ListNode helper function
    fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vector.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn test_delete_duplicates_case_1() {
        let input = to_list(vec![1, 1, 2]);
        let expected = to_list(vec![1, 2]);
        assert_eq!(Solution::delete_duplicates(input), expected);
    }

    #[test]
    fn test_delete_duplicates_case_2() {
        let input = to_list(vec![1, 1, 2, 3, 3]);
        let expected = to_list(vec![1, 2, 3]);
        assert_eq!(Solution::delete_duplicates(input), expected);
    }

    #[test]
    fn test_delete_duplicates_case_empty() {
        assert_eq!(Solution::delete_duplicates(None), None);
    }

    #[test]
    fn test_delete_duplicates_all_same() {
        let input = to_list(vec![1, 1, 1]);
        let expected = to_list(vec![1]);
        assert_eq!(Solution::delete_duplicates(input), expected);
    }
}

// MEMO:
//
// Iterate until the next node is None
// If the current node's value equals the next node's value,
// skip the next node by pointing to the "next of next" node.
// Otherwise, move the current pointer to the next node.
