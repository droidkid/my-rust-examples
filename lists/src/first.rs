use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> List {
        List {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, x: i32) {
        let next_node = Node {
            elem: x,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(next_node));
    }
}


// ============ NOTES
/*
pub enum List {
    Empty,
    Elem(i32, List),
}

This is invalid, this is like saying

struct A {
    A;
}

in C.
*/



