Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
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

作者：nibin
链接：https://leetcode-cn.com/problems/palindrome-linked-list/solution/rustkai-shi-xue-rustyong-de-ben-ban-fa-by-nibin/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
