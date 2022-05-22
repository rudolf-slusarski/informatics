use std::{
    cell::RefCell,
    cmp,
    rc::{Rc, Weak},
};

enum Color {
    Red,
    Black,
}

type Child<T> = RefCell<Option<Rc<Node<T>>>>;
type Parent<T> = RefCell<Option<Weak<Node<T>>>>;

struct Node<T> {
    value: Option<T>,
    color: RefCell<Color>,
    left: Child<T>,
    right: Child<T>,
    parent: Parent<T>,
}

impl<T> Node<T> {
    fn new() -> Self {
        Self {
            value: None,
            color: RefCell::new(Color::Black),
            left: RefCell::new(None),
            right: RefCell::new(None),
            parent: RefCell::new(None),
        }
    }

    fn new_from(n: T, color: Color) -> Self {
        Node {
            value: Some(n),
            color: RefCell::new(color),
            left: RefCell::new(None),
            right: RefCell::new(None),
            parent: RefCell::new(None),
        }
    }

    fn height(&self) -> usize {
        let left_height = match *self.left.borrow() {
            Some(ref left) => left.height(),
            None => 0,
        };
        let right_height = match *self.right.borrow() {
            Some(ref right) => right.height(),
            None => 0,
        };
        cmp::max(left_height, right_height) + 1
    }
}

struct Tree<T> {
    root: Rc<Node<T>>,
    count: usize,
}

impl<T> Tree<T> {
    fn new() -> Self {
        Self {
            root: Rc::new(Node::new()),
            count: 0,
        }
    }

    fn height(&self) -> usize {
        if self.count == 0 {
            0
        } else {
            self.root.height()
        }
    }
}
