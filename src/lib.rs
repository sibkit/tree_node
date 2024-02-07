pub mod tree_node;



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
