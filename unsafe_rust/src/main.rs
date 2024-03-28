use std::{fmt::Debug, ptr::NonNull};

#[derive(Debug, Copy, Clone)]
struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}
struct LL<T> {
    head: Option<NonNull<Node<T>>>,
    length: usize,
}
impl<T: Ord + Debug + Copy> LL<T> {
    pub fn new() -> LL<T> {
        LL {
            head: None,
            length: 0,
        }
    }
    pub fn insert(&mut self, data: T) {
        let new_node = NonNull::new(&mut Node { data, next: None } as *mut Node<T>);
        if self.head.is_none() {
            self.head = new_node;
        } else {
            unsafe { (*new_node.unwrap().as_ptr()).next = self.head }
        }
        self.length += 1;
    }
    pub fn find(&self, data: T) -> Option<&Node<T>> {
        match self.length {
            0 => return None,
            1 => unsafe {
                if (*self.head.unwrap().as_ptr()).data == data {
                    return Some(&*self.head.unwrap().as_ptr());
                } else {
                    return None;
                }
            },
            _ => {
                unsafe {
                    let mut curr = &(*self.head.unwrap().as_ptr());
                    for _ in 0..self.length {
                        println!("{:?}", curr);
                        if curr.data == data {
                            return Some(&curr);
                        }
                        curr = &(*curr.next.unwrap().as_ptr());
                    }
                }
                None
            }
        }
    }
    pub fn print(&self) {
        let mut curr = unsafe { *self.head.unwrap().as_ptr() };
        for _ in 0..self.length {
            println!("{:?}", curr);
        }
    }
}
fn main() {
    let mut ll = LL::new();
    ll.insert(10);
    ll.insert(20);
    ll.insert(30);
    ll.insert(40);
    ll.insert(50);
    ll.insert(60);
    ll.print()
}
#[cfg(test)]
mod test {
    use crate::LL;

    #[test]
    fn insert() {
        let mut ll = LL::new();
        ll.insert(10);
        unsafe {
            assert_eq!((*ll.head.unwrap().as_ptr()).data, 10);
        }
        ll.insert(20);
        unsafe {
            assert_eq!((*ll.head.unwrap().as_ptr()).data, 20);
        }
    }
    #[test]
    fn find() {
        let mut ll = LL::new();
        ll.insert(10);
        ll.insert(20);
        unsafe {
            assert_eq!((*ll.head.unwrap().as_ptr()).data, 20);
        }
    }
}
