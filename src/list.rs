use std::mem;
use std::fmt;

// a.k.a stack
pub struct List{
    head: Link
}

pub struct Node{
    elem: i32,
    next: Link,
}

enum Link{
    Empty,
    More(Box<Node>),
}


impl List {
    pub fn new() -> Self{
        List {head: Link::Empty}
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Node{
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        let new_node = Box::new(new_node);
        // set the head
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(s) => {
                self.head = s.next;
                Some(s.elem)
            }
        }
    }
}