pub struct TreeNode<T> {
    value: T,
    children: Vec<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            value: val,
            children: Vec::new(),
        }
    }

    pub fn val(&self) -> &T {
        &self.value
    }

    pub fn children(&self) -> &Vec<Box<TreeNode<T>>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Box<TreeNode<T>>> {
        &mut self.children
    }

    pub fn add_node(&mut self, val: T) {
        let new: Box<TreeNode<T>> = Box::new(TreeNode::new(val));
        self.children.push(new);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_val() {
        let root: TreeNode<i32> = TreeNode::new(37);
        assert_eq!(*root.val(),37);
    }
}