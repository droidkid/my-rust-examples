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

impl<T> Node<T> {
    fn get_next_node(& self) -> Option<& Self> {
        /*
           match self.next {
           Some(ref boxed_node) => {
           Some(boxed_node)
           }
           None => None
           }
           */

        self.next.as_ref().map(
            |ref_boxed_node| {
                &** ref_boxed_node // Twice because we used as_ref
            }
            )
    }

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

        head_option_boxed_node.map(
            |mut boxed_node| {
                self.head = boxed_node.next.take();
                boxed_node.elem
            }
            )
    }
}

pub struct LinkedListIter<'a, T:'a> {
    next:  Option<&'a Node<T>>,
}

impl <T> List<T> {
    pub fn get_iterator(& self) -> LinkedListIter<T> {
        LinkedListIter {
            next: self.head.as_ref().map(
                      |ref_boxed_node| {
                          /*
                           * First deref for ref
                           * Second deref for box
                           * Now we've got our node
                           * Return a reference to Node
                           *
                           * &ref_boxed_node would return a reference to the Box.
                           * We want reference to Node.
                           */
                          &**ref_boxed_node
                      }
                      )
        }
    }
}


impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;


    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(ref_to_node) => { // next is of type Option<& Node>
                self.next = ref_to_node.get_next_node();
                Some(& ref_to_node.elem)
            }
            None => None,
        }
    }
}

pub struct LinkedListIterMut<'a, T:'a> {
    next: Option<&'a mut Node<T>>,
}

impl <T> List<T> {
    pub fn get_mut_iterator(&mut self) -> LinkedListIterMut<T> {
        LinkedListIterMut {
            next: self.head.as_mut().map(
                      |mut_ref_boxed_node| {
                          &mut **mut_ref_boxed_node
                      }
                      )
        }
    }
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(boxed_node) => {
                self.next = boxed_node.next.as_mut().map(
                        |mut_ref_boxed_node| {
                            &mut **mut_ref_boxed_node
                        }
                    );
                Some(&mut boxed_node.elem)
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

#[test]
    fn iter() {
        let mut l: List<i32> = List::new();

        assert_eq!(l.pop(), None);

        l.push(1);
        l.push(2);
        l.push(3);


        {
            let mut list_iterator = l.get_iterator();
            assert_eq!(list_iterator.next(), Some(&3));
            assert_eq!(list_iterator.next(), Some(&2));
            assert_eq!(list_iterator.next(), Some(&1));
            assert_eq!(list_iterator.next(), None);
        }

        l.push(5);
    }

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.get_mut_iterator();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
