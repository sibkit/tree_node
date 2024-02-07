use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode<T> {
    core: Rc<RefCell<NodeCore<T>>>
}

//pub type TreeNode<T> = Rc<RefCell<NodeCore<T>>>;
#[derive(Debug)]
pub struct NodeCore<T>{
    value: T,
    parent: Option<TreeNode<T>>,
    child_nodes: Vec<TreeNode<T>>
}

impl <T> TreeNode<T> {
    pub fn new(value: T, parent: Option<TreeNode<T>>) -> Self {

        let result = TreeNode::<T> {
            core: Rc::new(RefCell::new(NodeCore {
                value,
                parent: parent.clone(),
                child_nodes: vec![],
            }))
        };

        if parent.is_some() {
            parent.unwrap().core.borrow_mut().child_nodes.push(result.clone());
        }

        return result;
    }
    /*
        fn get_core(&mut self) -> RefMut<NodeCore<T>> {
            match &mut self.var {
                RcVar::Rc(rc) => {
                    rc.borrow_mut()
                }
                RcVar::Weak(weak) => {
                    weak.upgrade().unwrap().borrow_mut()
                }
            }
        }
    */
    pub fn add_child(&mut self, node: TreeNode<T>) -> Result<(), String> {
        node.core.borrow_mut().parent = Some(self.clone());
        self.core.borrow_mut().child_nodes.push(node);
        Ok(())
    }

    pub fn remove_child(&mut self, index: usize) -> Result<TreeNode<T>, String> {
        Ok(self.core.borrow_mut().child_nodes.remove(index))
    }
}

impl <T> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        TreeNode{
            core: self.core.clone()
        }
    }
}