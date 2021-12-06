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
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder(node.borrow_mut().left.take(), ret);
                ret.push(node.borrow().val);
                inorder(node.borrow_mut().right.take(), ret);
            }
        }

        inorder(root, &mut ans);
        ans
    }
}
