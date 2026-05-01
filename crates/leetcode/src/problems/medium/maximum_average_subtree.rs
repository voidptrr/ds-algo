// https://leetcode.com/problems/maximum-average-subtree

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
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut f64) -> (f64, f64) {
        let Some(node_rc) = node else {
            return (0.0, 0.0); // (sum, count)
        };

        let node_ref = node_rc.borrow();
        let (left_sum, left_count) = Self::dfs(&node_ref.left, best);
        let (right_sum, right_count) = Self::dfs(&node_ref.right, best);

        let sum = node_ref.val as f64 + left_sum + right_sum;
        let count = 1.0 + left_count + right_count;
        let avg = sum / count;
        *best = best.max(avg);

        (sum, count)
    }

    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        let mut best = f64::MIN;
        let _ = Self::dfs(&root, &mut best);
        best
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
