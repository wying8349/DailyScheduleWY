// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut stack : Vec<i32> = Vec::new();
        let mut list = &head;
        loop {
            if let Some(ref tmp) = list {
                stack.push(tmp.val);
                println!("{}", tmp.val);
                list = &tmp.next;
            } else {
               break;
            }       
        }

        let mut compare : Vec<i32> = stack.clone();
        compare.reverse();

        if stack == compare {
            return true;
        }

        false
    }
}
