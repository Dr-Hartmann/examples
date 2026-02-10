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

pub fn add_two_numbers(l1: i32, l2: i32) -> Box<ListNode> {
    let l1 = digits_to_mod(l1);
    let l2 = digits_to_mod(l2);
    match Solution::add_two_numbers(l1, l2) {
        Some(value) => value,
        None => Box::new(ListNode::new(0)),
    }
}

pub fn list_to_i32(l: Option<Box<ListNode>>) -> i32 {
    node_to_i32(l, 1)
}

fn node_to_i32(l: Option<Box<ListNode>>, mul: u32) -> i32 {
    if let Some(value) = l {
        return value.val * mul as i32 + node_to_i32(value.next, mul * 10);
    }
    0
}

fn digits_to_mod(l: i32) -> Option<Box<ListNode>> {
    if l <= 0 {
        return None;
    }

    let mut out = Box::new(ListNode::new(l % 10));
    out.next = digits_to_mod(l / 10);
    Some(out)
}

struct Solution;

impl Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::solve(l1, l2, 0)
    }

    fn solve(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let mut sum = carry;
        let mut next_l1 = None;
        let mut next_l2 = None;

        if let Some(node) = l1 {
            sum += node.val;
            next_l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            next_l2 = node.next;
        }

        let mut current_node = Box::new(ListNode::new(sum % 10));
        current_node.next = Self::solve(next_l1, next_l2, sum / 10);
        Some(current_node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers_test() {
        let out = add_two_numbers(15146, 14854);
        assert_eq!(30_000, list_to_i32(Some(out)));
    }
}
