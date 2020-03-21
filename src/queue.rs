use std::mem;
use std::rc::Rc;
use std::fmt;


pub struct Queue{
    front: Link,
    rear: Link,
    empty: bool,
}

pub struct Node{
    elem: i32,
    next: Link,
}

enum Link{
    Empty,
    More(Rc<Node>),
}

// FRONT -> REAR
// FRONT -> [A] -> REAR
// FRONT -> [A] -> [B] -> [C] -> REAR
impl Queue {
    pub fn new() -> Self{
        Queue {front: Link::Empty, rear: Link::Empty, empty: true}
    }

    pub fn push(&mut self, elem: i32){
        if self.empty {
            // The front node should have a link to this node
            let new_node = Node{
                elem,
                next: mem::replace(&mut self.rear, Link::Empty),
            };
            let new_node = Rc::new(new_node);

            self.front = Link::More(Rc::clone(&new_node));
            self.rear = Link::More(Rc::clone(&new_node));
            self.empty = false;
        }else{

            let new_node = Node{
                elem,
                next: mem::replace(&mut self.rear, Link::Empty),
            };
            let new_node = Rc::new(new_node);
            self.rear = Link::More(new_node);
        }
    }

    // Pop from the front and change its pointer
    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.front, Link::Empty){
            Link::Empty => None,
            Link::More(s) => {
                let junk = Rc::clone(&s);
                self.front = Rc::try_unwrap(s).ok().unwrap().next;
                Some(junk.elem)
            }
        }
    }
}