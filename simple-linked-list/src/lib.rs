use std::iter::FromIterator;
use std::mem;


pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::clone::Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        if self.head.is_none() {
            return 0;
        }
        let mut this = self.head.as_ref().unwrap().next.as_ref();
        let mut len: usize = 1;
        loop {
            if this.is_none() {
                return len;
            }
            len += 1;
            this = this.unwrap().next.as_ref();
        }
    }

    pub fn push(&mut self, _element: T) {
        let mut head = None;
        mem::swap(&mut self.head, &mut head);

        let n = Node {
            data: _element,
            next: head,
        };

        *self = SimpleLinkedList {
            head: Some(Box::new(n)),
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = None;
        mem::swap(&mut self.head, &mut head);
        //if head.as_ref().unwrap().is_none() { return None }

        let c = head.as_ref();

        if c.is_none() {
            return None
        }

        let r = c.unwrap();
        
        if r.next.is_none() {
            return None
        }

        let s = r.data.clone();

        *self = SimpleLinkedList {
            head: Some(head.unwrap().next.unwrap()),
        };

        Some(s)
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
