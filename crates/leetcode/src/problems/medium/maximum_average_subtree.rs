// https://leetcode.com/problems/maximum-average-subtree

use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        let Some(root) = root else {
            return 0f64;
        };

        let mut stack = vec![(root, false)];
        let mut subtree: HashMap<*const RefCell<TreeNode>, (f64, f64)> =
            HashMap::new();
        let mut result = f64::MIN;

        while let Some((node, visited)) = stack.pop() {
            if visited {
                let n = node.borrow();

                let (left_sum, left_count) = if let Some(left) = &n.left {
                    subtree
                        .get(&Rc::as_ptr(left))
                        .copied()
                        .unwrap_or((0.0, 0.0))
                } else {
                    (0.0, 0.0)
                };

                let (right_sum, right_count) = if let Some(right) = &n.right {
                    subtree
                        .get(&Rc::as_ptr(right))
                        .copied()
                        .unwrap_or((0.0, 0.0))
                } else {
                    (0.0, 0.0)
                };

                let sum = n.val as f64 + left_sum + right_sum;
                let nodes_count = 1.0 + left_count + right_count;
                let avg = sum / nodes_count;
                result = result.max(avg);

                subtree.insert(Rc::as_ptr(&node), (sum, nodes_count));
            } else {
                stack.push((Rc::clone(&node), true));
                let n = node.borrow();

                if let Some(right) = &n.right {
                    stack.push((Rc::clone(right), false));
                }
                if let Some(left) = &n.left {
                    stack.push((Rc::clone(left), false));
                }
            }
        }

        result
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
        let root = node(5, node(6, None, None), node(1, None, None));

        assert!((Solution::maximum_average_subtree(root) - 6.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let root = node(0, None, None);

        assert!((Solution::maximum_average_subtree(root) - 0.0).abs() < 1e-5);
    }
}
