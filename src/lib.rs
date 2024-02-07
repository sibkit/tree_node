pub mod tree_node;

pub use crate::tree_node::TreeNode;
pub use crate::tree_node::NodeCore;
#[cfg(test)]
mod tests {
    use crate::tree_node::TreeNode;
    use super::*;

    #[test]
    fn it_works() {
        let mut node = TreeNode::new("str",None);
        println!("{:?}", node)
    }
}
