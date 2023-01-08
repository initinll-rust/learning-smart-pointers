// Strong reference - Rc<T> - 
//// a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well.
// Weak reference - Weak<T> - 
//// a child should not own its parent: if we drop a child node, the parent should still exist.

use std::{cell::RefCell, rc::{Weak, Rc}};

#[derive(Debug)]
struct Node {
    value: i32, // value of the node
    parent: RefCell<Weak<Node>>, // a node has single parent, weak reference as child should not own its parent 
    children: RefCell<Vec<Rc<Node>>> // a node can have multiple children, strong reference as parent should own its children
}

pub fn create_tree() {
    println!("\nPrevent Reference Cycles - Rc<T>, Weak<T> & RefCell<T>");

    let leaf_node_1 = Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    };

    let leaf_node_2 = Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    };

    let leaf_1 = Rc::new(leaf_node_1);
    let leaf_2 = Rc::new(leaf_node_2);

    println!("\nBefore Relation");
    println!("leaf 1 parent = {:?}", leaf_1.parent.borrow().upgrade());
    println!("leaf 2 parent = {:?}", leaf_2.parent.borrow().upgrade());

    println!("leaf 1 strong = {}, weak = {}", Rc::strong_count(&leaf_1), Rc::weak_count(&leaf_1));
    println!("leaf 2 strong = {}, weak = {}", Rc::strong_count(&leaf_2), Rc::weak_count(&leaf_2));

    let root_node = Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf_1), Rc::clone(&leaf_2)])
    };

    let root = Rc::new(root_node);

    println!("\nroot = {:?}", root);

    println!("root strong = {}, weak = {}", Rc::strong_count(&root), Rc::weak_count(&root));

    *leaf_1.parent.borrow_mut() = Rc::downgrade(&root);
    *leaf_2.parent.borrow_mut() = Rc::downgrade(&root);

    println!("\nAfter Relation");
    println!("leaf 1 parent = {:?}", leaf_1.parent.borrow().upgrade());
    println!("leaf 2 parent = {:?}", leaf_2.parent.borrow().upgrade());

    println!("leaf 1 strong = {}, weak = {}", Rc::strong_count(&leaf_1), Rc::weak_count(&leaf_1));
    println!("leaf 2 strong = {}, weak = {}", Rc::strong_count(&leaf_2), Rc::weak_count(&leaf_2));
}
