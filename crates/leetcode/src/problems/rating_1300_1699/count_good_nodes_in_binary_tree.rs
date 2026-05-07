// https://leetcode.com/problems/count-good-nodes-in-binary-tree

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[must_use]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };

        let mut count = 0;
        let root_max = root.borrow().val;
        let mut stack = vec![(root, root_max)];

        while let Some((node, path_max)) = stack.pop() {
            let borrowed = node.borrow();

            if borrowed.val >= path_max {
                count += 1;
            }

            let next_max = borrowed.val.max(path_max);

            if let Some(left) = &borrowed.left {
                stack.push((Rc::clone(left), next_max));
            }

            if let Some(right) = &borrowed.right {
                stack.push((Rc::clone(right), next_max));
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn example_one() {
        let root = node(
            3,
            node(1, node(3, None, None), None),
            node(4, node(1, None, None), node(5, None, None)),
        );

        assert_eq!(Solution::good_nodes(root), 4);
    }

    #[test]
    fn example_two() {
        let root =
            node(3, node(3, node(4, None, None), node(2, None, None)), None);

        assert_eq!(Solution::good_nodes(root), 3);
    }

    #[test]
    fn example_three() {
        let root = node(1, None, None);

        assert_eq!(Solution::good_nodes(root), 1);
    }
}
