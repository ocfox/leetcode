use std::cell::RefCell;
use std::rc::Rc;
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
pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let root = root.borrow();
            let val = root.val.to_string();
            return match (root.left.clone(), root.right.clone()) {
                (None, None) => val,
                (_, None) => format!("{}({})", val, dfs(root.left.clone())),
                (_, _) => format!(
                    "{}({})({})",
                    val,
                    dfs(root.left.clone()),
                    dfs(root.right.clone())
                ),
            };
        }
        "".to_string()
    }
    dfs(root)
}
fn main() {}
