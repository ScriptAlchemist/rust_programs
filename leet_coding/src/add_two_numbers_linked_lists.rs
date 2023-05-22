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

#[allow(dead_code)]
pub fn add_two_numbers_v1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let (mut p, mut q, mut carry, mut current) = (l1, l2, 0 , &mut dummy_head);

    while p.is_some() || q.is_some() {
        let sum = carry
            + p.as_ref().map_or(0, |node| node.val)
            + q.as_ref().map_or(0, |node| node.val);
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        p = p.and_then(|node| node.next);
        q = q.and_then(|node| node.next);
    }

    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }
    dummy_head.next
}

#[cfg(test)]
mod add_two_numbers {
    use super::*;
    #[test]
    fn test_add_two_numbers_v1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = add_two_numbers_v1(l1, l2);
        assert!(result.is_some());
        let mut result = result.unwrap();
        assert_eq!(result.val, 7);
        assert!(result.next.is_some());
        result = result.next.unwrap();

        assert_eq!(result.val, 0);
        assert!(result.next.is_some());
        result = result.next.unwrap();

        assert_eq!(result.val, 8);
        assert!(result.next.is_none());
    }
}

#[allow(dead_code)]
pub fn add_two_numbers_v2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let (mut p, mut p1, mut p2, mut carry) = (&mut head, &l1, &l2, 0);
    while p1.is_some() || p2.is_some() {
        let mut sum = carry;
        if let Some(node) = p1 {
            sum += node.val;
            p1 = &node.next;
        }
        if let Some(node) = p2 {
            sum += node.val;
            p2 = &node.next;
        }
        carry = sum / 10;
        if let Some(node) = p {
            node.next = Some(Box::new(ListNode::new(sum % 10)));
            p = &mut node.next;
        }
        if carry > 0 {
            if let Some(node) = p {
                node.next = Some(Box::new(ListNode::new(carry)));
            }
        }
    }
    head.unwrap().next
}

#[cfg(test)]
mod add_two_numbers_two {
    use super::*;
    #[test]
    fn test_add_two_numbers_v2() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let result = add_two_numbers_v2(l1, l2);
        assert!(result.is_some());
        let mut result = result.unwrap();
        assert_eq!(result.val, 7);
        assert!(result.next.is_some());
        result = result.next.unwrap();

        assert_eq!(result.val, 0);
        assert!(result.next.is_some());
        result = result.next.unwrap();

        assert_eq!(result.val, 8);
        assert!(result.next.is_none());
    }
}
