// Definition for a binary tree node.
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut n_of_paths: i32 = 0;
        let mut stack = vec![(root, [0; 10])];

        while let Some((node, mut counter)) = stack.pop() {
            if let Some(node_rc) = node {
                let mut node = node_rc.borrow_mut();
                counter[node.val as usize] += 1;

                if node.left.is_none() && node.right.is_none() {
                    n_of_paths += (counter.iter().filter(|&&x| x % 2 == 1).count() <= 1) as i32;
                    continue;
                }
                
                stack.push((node.left.take(), counter));
                stack.push((node.right.take(), counter));
            }
        }

        n_of_paths
    }
}

pub fn tests() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        }))),
    })));

    assert_eq!(2, Solution::pseudo_palindromic_paths(root));
}