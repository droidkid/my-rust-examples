/*
 * Empty List:
 *
 * Stack        [head: None]
 *
 *
 * 2 Elements
 *
 * Stack        [head: Some(Box-> address1)]   List
 * address1     [elem: 1, next: Some(Box->address2)] Node (address1 stores a Node).
 * address2     [elem: 3, next: None]                Node (address2 stores a Node).
 *
 *
 * Think in terms of "References"!
 *
 *
 *
 * If we went with 
 *
 * enum List {
 *  Empty,
 *  Enum(i32, Box<List>)
 * }
 *
 * Empty List
 *
 *              [Empty, *extra space for Box and i32*]
 *
 * List 1 with 1 element
 *
 * Stack        [ (5, Box->address1) ]
 * address1     [ (6, Box->address2) ]
 * address2     [ (1, Box->address3) ]
 * address3     [ Empty, *extra space for Box and i32* ]
 *
 *
 * Box will allocate a element on heap and store a pointer to that.
 */

use std::mem;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>; 

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl <T> List <T> {
    fn new() -> Self {
        List {
            head: None,
        }
    }

    fn push(&mut self, val: T) {
        let n: Node<T> = Node {
            elem: val,

            /**
             * v1:
             *
             * next: self.head
             *
             * Does not work because this is transeferring ownership of self.head.
             * But self.head is just a borrowed value (see the &mut self parameter)
             *
             * v2:
             *
             * next: mem::replace(&mut self.head, None)
             *
             * Works!
             *
             * v3:
             *
             * next: self.head.take() // self.head becomes None.
             */

            next: self.head.take(),
        };
        self.head = Some(Box::new(n));
    }

    fn pop(&mut self) -> Option<T> {
        // We have a mutable reference to self.head.
        // match self.head {
            // v1:
            // Remember self.head is a mutable reference
            // Some(boxed_node) tries to take ownership of boxed_node, which is not
            // possible because we NEVER had ownership of self.head .
            //
            //Some(boxed_node) => {
            //
            // v2:
            // Let's try ref boxed_node... Nope, doesn't work.
            //
            // The problem is Some(boxed_node.elem) takes ownership of boxed_node.
            // The issue is we NEED ownership of boxed_node.
            // Some(ref boxed_node) => {
            //    Some(boxed_node.elem)
            // }
            // None => None,
        //}
        //


        // Make head empty and take ownership.
        // head_option_boxed_node will be dropped when this stack ends.
        let head_option_boxed_node = self.head.take();         

        match head_option_boxed_node {
            Some(mut boxed_node) => {
                self.head = boxed_node.next.take();
                Some(boxed_node.elem)
            }
            None => None,
        }


    }
}

#[test]
fn basic() {
    
    let mut l: List<i32> = List::new();

    assert_eq!(l.pop(), None);

    l.push(1);
    l.push(2);
    l.push(3);
    
    assert_eq!(l.pop(), Some(3));
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), Some(1));

}
