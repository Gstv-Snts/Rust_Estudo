use std::ptr::null_mut;

pub struct Queue<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

pub type Link<T> = *mut Node<T>;

pub struct Node<T: Clone> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T: Clone>(Queue<T>);

pub struct Iter<'a, T: Clone> {
    next: &'a Link<T>,
}

pub struct IterMut<'a, T: Clone> {
    next: &'a mut Link<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let new = Box::into_raw(Box::new(Node {
            elem,
            next: null_mut(),
        }));
        match self.len {
            0 => self.head = new,
            1 => unsafe {
                (*self.head).next = new;
                self.tail = new;
            },
            _ => unsafe {
                (*self.tail).next = new;
                self.tail = new;
            },
        }
        self.len += 1
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        if self.len == 1 {
            let h = self.head;
            self.head = null_mut();
            self.len -= 1;
            return unsafe { Some((*h).elem.clone()) };
        }
        let h = self.head;
        self.head = unsafe { (*h).next };
        self.len -= 1;
        unsafe { Some((*h).elem.clone()) }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len > 0 {
            return unsafe { Some(&(*self.head).elem) };
        }
        None
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        if self.len > 0 {
            return unsafe { Some(&mut (*self.head).elem) };
        }
        None
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: &self.head }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: &mut self.head,
        }
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next;
        self.next = unsafe { &(*(*n)).next };
        unsafe { Some(&(*(*n)).elem) }
    }
}

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let n = *self.next;
        self.next = unsafe { &mut (*n).next };
        unsafe { Some(&mut (*n).elem) }
    }
}

#[cfg(test)]
mod test {
    use std::ptr::null_mut;

    use super::Queue;

    #[test]
    fn new() {
        let q: Queue<usize> = Queue::new();
        assert_eq!(q.len, 0);
        assert_eq!(q.head, null_mut());
        assert_eq!(q.tail, null_mut());
    }

    #[test]
    fn enqueue() {
        let mut q: Queue<usize> = Queue::new();
        q.enqueue(10);
        unsafe { assert_eq!((*q.head).elem, 10) };
        q.enqueue(20);
        unsafe {
            assert_eq!((*q.head).elem, 10);
            assert_eq!((*q.tail).elem, 20)
        };
        q.enqueue(30);
        unsafe {
            assert_eq!((*q.head).elem, 10);
            assert_eq!((*q.tail).elem, 30)
        };
    }

    #[test]
    fn dequeue() {
        let mut q: Queue<usize> = Queue::new();
        assert!(q.dequeue().is_none());
        q.enqueue(10);
        unsafe { assert_eq!(q.dequeue().unwrap(), 10) };
        q.enqueue(20);
        q.enqueue(30);
        unsafe { assert_eq!(q.dequeue().unwrap(), 20) };
        q.enqueue(40);
        q.enqueue(50);
        q.enqueue(60);
        unsafe { assert_eq!(q.dequeue().unwrap(), 30) };
    }

    #[test]
    fn peek() {
        let mut q: Queue<usize> = Queue::new();
        assert_eq!(q.peek(), None);
        q.enqueue(10);
        assert_eq!(q.peek(), Some(&10));
        q.dequeue();
        assert_eq!(q.peek(), None);
        q.enqueue(10);
        q.enqueue(20);
        assert_eq!(q.peek(), Some(&10));
    }

    #[test]
    fn peek_mut() {
        let mut q: Queue<usize> = Queue::new();
        assert_eq!(q.peek_mut(), None);
        q.enqueue(10);
        assert_eq!(q.peek_mut(), Some(&mut 10));
        q.dequeue();
        assert_eq!(q.peek_mut(), None);
        q.enqueue(10);
        q.enqueue(20);
        assert_eq!(q.peek_mut(), Some(&mut 10));
    }

    #[test]
    fn into_iter() {
        let mut q: Queue<usize> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        q.enqueue(40);
        q.enqueue(50);
        let mut ii = q.into_iter();
        assert_eq!(ii.next().unwrap(), 10);
        assert_eq!(ii.next().unwrap(), 20);
        assert_eq!(ii.next().unwrap(), 30);
        assert_eq!(ii.next().unwrap(), 40);
        assert_eq!(ii.next().unwrap(), 50);
    }

    #[test]
    fn iter() {
        let mut q: Queue<usize> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        q.enqueue(40);
        q.enqueue(50);
        let mut i = q.iter();
        assert_eq!(i.next().unwrap(), &10);
        assert_eq!(i.next().unwrap(), &20);
        assert_eq!(i.next().unwrap(), &30);
        assert_eq!(i.next().unwrap(), &40);
        assert_eq!(i.next().unwrap(), &50);
    }

    #[test]
    fn iter_mut() {
        let mut q: Queue<usize> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);
        q.enqueue(40);
        q.enqueue(50);
        let mut i = q.iter_mut();
        assert_eq!(i.next().unwrap(), &mut 10);
        assert_eq!(i.next().unwrap(), &mut 20);
        assert_eq!(i.next().unwrap(), &mut 30);
        assert_eq!(i.next().unwrap(), &mut 40);
        assert_eq!(i.next().unwrap(), &mut 50);
    }
}
