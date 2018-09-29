

mod tree;
mod party;


#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use tree::TreeNode;
use party::Party;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

use std::string::String;

/// App entry poing
fn main() {

    // logger initialization
    env_logger::Builder::from_env("APP_LOG").init();

    // create a new tree node
    let root:Rc<TreeNode<Party>> = Rc::new(TreeNode::<Party> {
        value: Rc::new(Party::new(236, "COBA")),
        children: RefCell::new(vec![]),
        parent: Weak::new()
    });

    info!("My tree node is: {:?}", root);

    TreeNode::add_child(&root, Party::new(21, "BAS"));
    let new_node = TreeNode::add_child(&root, Party::new(22, "BAS"));
    let new_node2 = TreeNode::add_child(&new_node, Party::new(1001, "THOUSAND AND ONE"));
    TreeNode::add_child(&new_node2, Party::new(99, "NINETY NINE PLC"));
    TreeNode::add_child(&root, Party::new(77, "SEVENTY SEVEN SRL"));
    info!("My tree node is: {:?}", &root);

    log_tree(&root, 0);

    serde_party();


}

    /// Simple Serde of a party
fn serde_party() {

    let p =  Party::new(22, "BAS");
    println!("### Serde JSON serialized ###");

    let serialized = serde_json::to_string(&p).unwrap();
    println!("{}", serialized);


}

/// Just create some log output from the node
fn log_tree(node : &Rc<TreeNode<Party>>, indent: i32 ) {

    // A bit slow -- allocating Strings ..TreeNode
    let mut s = String::new();
    for _i in 0.. indent {
        s = s + "\t";
    }

    // TODO: why does node.get_parent_path() "just work" - deref?
    // Look at Rc docs to find out
    info!("{} : {}[{}] {:?}", s, &node.value.org_id, &node.value.legal_name, node.get_parent_path());
    let vec = node.children.borrow();
    for x in vec.iter()  {
        log_tree(x, indent+1);
    }


}