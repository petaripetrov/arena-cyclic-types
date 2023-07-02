use std::{cell::Cell, fmt::Display};

use crate::Arena;

/// Where the magic happens, this type signature allows for cyclic types
pub type Link<'a, T> = Cell<Option<&'a T>>;

/// Simple Cyclic Node
pub struct Node<'a, T: Display> {
    pub data: T,
    pub parent: Link<'a, Node<'a, T>>,
    pub right: Link<'a, Node<'a, T>>,
    pub left: Link<'a, Node<'a, T>>,
}

/// Basic linked list with back reference to the previous element.
pub struct List<'a, T: Display> {
    pub head: Link<'a, Node<'a, T>>,
}

impl<'a, T: Display> List<'a, T> {
    pub fn push(&self, new_node: &'a Node<'a ,T>) {
        if let Some(node) = self.head.get() {
            let mut curr = node;

            loop {
                match curr.right.get() {
                    Option::Some(x) => curr = x,
                    Option::None => break
                };
            }

            new_node.left.set(Some(curr));
            curr.right.set(Some(new_node));
        } else {
            self.head.set(Some(new_node));
        }
    }

    pub fn print(&self) {
        let top = self.head.get();

        let mut temp = top;
        let mut prev_temp = top;

        while let Some(node) = temp {
            println!("Node with value :: {}", node.data);
            prev_temp = temp;
            temp = node.right.get();
        }
        
        temp = prev_temp;

        while let Some(node) = temp {
            println!("Node with value :: {}", node.data);
            temp = node.left.get();
        }
    }
}

/// Basic tree with back reference to the node parent.
pub struct Tree<'a, T: Display + Ord> {
    pub root: Link<'a, Node<'a, T>>,
}

impl<'a, T: Display + Ord> Tree<'a, T> {
    pub fn push(&self, new_node: &'a Node<'a, T>) {
        let top = self.root.replace(None);

        if top.is_none() {
            self.root.replace(Some(new_node));
            return;
        }

        let mut temp = top;

        while let Some(node) = temp {
            if new_node.data > node.data {
                if node.right.get().is_none() {
                    new_node.parent.set(Some(node));
                    node.right.set(Some(new_node));
                    break;
                }

                temp = node.right.get();
            } else {
                if node.left.get().is_none() {
                    new_node.parent.set(Some(node));
                    node.left.set(Some(new_node));
                    break;
                }

                temp = node.left.get();
            }
        }

        self.root.replace(top);
    }

    pub fn print(&self) {
        let root = self.root.get();

        println!("Tree:");
        Tree::print_in_order(root);
        println!();

        return;
    }

    fn print_in_order(root: Option<&Node<T>>) {
        match root {
            Option::Some(node) => {
                Tree::print_in_order(node.left.get());

                if let Some(parent) = node.parent.get() {
                    println!("Node: {} with parent: {}", node.data, parent.data);
                } else {
                    println!("Root: {}", node.data);
                }

                Tree::print_in_order(node.right.get());
            }
            Option::None => return,
        }
    }
}

/// Benchmark linked list with elements
/// # Arguments
/// * `n` - A usize for the capacity of the list and arena 
pub fn benchmark_linked_list_arena(n: usize) {
    let arena = Arena::new(n);
    let list = List {
        head: Cell::new(None),
    };

    for i in 0..n {
        let temp = arena.alloc(Node {
            data: i,
            parent: Cell::new(None),
            right: Cell::new(None),
            left: Cell::new(None),
        });
        list.push(temp);
    }
}
/// Benchmark linked list with elements
/// # Arguments
/// * `n` - A usize for the capacity of the arena
/// * arr - Generic vector, expected to be randomized. Non-randomized vectors 
///         would lead to a performance penalty due to the pushing algorithm 
pub fn benchmark_tree_arena<T: Display + Ord>(n: usize, arr: Vec<T>) {
    let arena = Arena::new(n);
    let tree = Tree {
        root: Cell::new(None),
    };

    for i in arr {
        let temp = arena.alloc(Node {
            data: i,
            parent: Cell::new(None),
            right: Cell::new(None),
            left: Cell::new(None),
        });
        tree.push(temp);
    }
}
