use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

//use std::cell::Cell;

type LinkNode<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    prev: LinkNode<T>, //identify real or nil next
    next: LinkNode<T>, //identify real or nil next
    val: T,
}

impl<T> fmt::Display for Node<T>
    where
        T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?}}}", self.val)
    }
}

impl<T> Node<T> {
    pub fn get_next(&self) -> LinkNode<T> {
        match self.next {
            None =>
                return None,
            // Not creating new mutable (unique!) references overlapping `element`.
            Some(node) =>
                return Some(node.clone()),
        }
    }

    pub fn set_next(&mut self, newnode: Rc<RefCell<Node<T>>>) {
        self.next = Some(newnode);
    }

    pub fn new(val: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(Node { next: None, val }));
    }
}

#[derive(Debug)]
pub struct List<T> {
    head: LinkNode<T>,
    tail: LinkNode<T>,
    size: u32,
}

impl<T: std::fmt::Debug> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut item = self.front();
        unsafe {
            while let Some(node) = item {
                println!("list: {:?}", node.borrow());
                //write!(f, "{{{0}}}", node.deref())
                //item = item.as_ref().borrow().get_next();
                item = node.borrow().get_next();
            }
            write!(f, "{{{0}}}", "end")
        }
    }
}

impl<T> List<T> {
    pub fn front(&self) -> LinkNode<T> {
        match &self.head {
            None =>
                return None,
            // Not creating new mutable (unique!) references overlapping `element`.
            Some(head) =>
                return Some(head.clone()),
        }
    }

    pub fn back(&self) -> LinkNode<T> {
        match &self.tail {
            None =>
                return None,
            // Not creating new mutable (unique!) references overlapping `element`.
            Some(tail) =>
                return Some(tail.clone()),
        }
    }

    pub fn push(&mut self, val: T) {
        let n = Node::new(val);
        if let Some(ref mut validnode) = self.tail {
            validnode.borrow_mut().set_next(n.clone());
            self.tail = Some(n.clone());
        }

        if let None = self.head {
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        }
        self.size += 1
    }
}

pub fn init_list<T>() -> List<T> {
    let newlist = List {
        size: 0,
        head: None,
        tail: None,
    };

    return newlist;
}
