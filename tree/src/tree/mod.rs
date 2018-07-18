
/// A simple struct for a tree, with backwards navigability to parent
/// As per rust book .. 
/// https://doc.rust-lang.org/book/second-edition/ch15-06-reference-cycles.html


use std::vec::Vec;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
// use std::fmt::Debug;

// try in ints first ?
#[derive(Debug)]
pub struct TreeNode<T> {

    // the value in the Tree
    pub value: Rc<T>, 

    /// children are "owned" by parent .. 
    /// Working  "inside-out"
    /// TreeNode -- yes, the child is another TreeNode
    /// Rc<TreeNode> .. so can have direct refs to each node from outside (via Rc)
    /// Vec<Rc<TreeNode>> ... to support multiple children
    /// RefCell ... so we can add children .. ("interior mutability pattern")
    pub children: RefCell<Vec<Rc<TreeNode<T>>>>,


    /// Parent .. needs to be what?
    /// guess .. ."weak" ... as not owned by parent (what about no parent at root ?) --Option ?
    /// RefCell ??
    pub parent: Weak<TreeNode<T>>

}



impl <T> TreeNode<T> {


    /// adds a child to a Node 
    pub fn add_child(node: &Rc<TreeNode<T>>, value: T) -> Rc<TreeNode<T>> {


        let result = Rc::new(TreeNode {
            value: Rc::new(value),
            children: RefCell::new(vec![]),
            parent: Rc::downgrade(&node)
        });

        // push new child onto th evec
        node.children.borrow_mut().push(result.clone());

        // set parent in the node ... 
        // how do we get week ref to self ?  -- do we need to pass

        //*result.parent.borrow_mut() = Rc::downgrade(&ccc);
        result
    }


    /// From a ref .. get the parent
    // #[inline]
    pub fn get_parent(&self) -> Option<Rc<TreeNode<T>>> {
        //.borrow().upgrade();
        self.parent.upgrade()
    }


    /// this ony works for Copy (i32) .. would need to Rc everything
    pub fn get_parent_path(&self) -> Vec<Rc<T>> {
        let mut result : Vec<Rc<T>> = Vec::new();

        result.push(Rc::clone(&self.value));

        // //Option<Rc<TreeNode
        let mut parent = self.get_parent();

         while parent.is_some() {

            let node = parent.unwrap();
            result.push(Rc::clone(&node.value));

            parent = node.get_parent();

        }

        result

    }

}


impl <T> Drop for TreeNode<T> {

    fn drop(&mut self) {
        // Can we check for "debuggable"
        trace!("Dropping" );
    }

}