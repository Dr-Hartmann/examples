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

pub fn add_two_numbers(l1: i32, l2: i32) {
    let l1 = rec_mod_digits(l1);
    let l2 = rec_mod_digits(l2);
    println!("{l1:?}");
    println!("{l2:?}");
    let out = match Solution::add_two_numbers(l1, l2) {
        Some(value) => value,
        None => Box::new(ListNode::new(0)),
    };
    println!("{out:#?}");
}

fn rec_mod_digits(l: i32) -> Option<Box<ListNode>> {
    if l <= 0 {
        return None;
    }

    let mut out = Box::new(ListNode::new(l % 10));
    out.next = rec_mod_digits(l / 10);
    Some(out)
}

struct Solution {}

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
