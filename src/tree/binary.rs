use crate::tree::node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BinaryTree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn insert(&mut self, val: isize) {
        let node = TreeNode::new(val);

        if self.root.is_none() {
            self.root = Some(node);
            return;
        }

        let mut current = self.root.clone();

        while let Some(curr_rc) = current {
            let mut curr = curr_rc.borrow_mut();

            if val < curr.val {
                if let Some(left) = curr.left.clone() {
                    current = Some(left);
                } else {
                    curr.left = Some(node);
                    break;
                }
            } else if let Some(right) = curr.right.clone() {
                current = Some(right);
            } else {
                curr.right = Some(node);
                break;
            }
        }
    }

    fn inorder(node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            Self::inorder(n.borrow().left.clone());
            print!("{} ", n.borrow().val);
            Self::inorder(n.borrow().right.clone());
        }
    }
}
