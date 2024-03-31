use std::mem;

#[derive(Debug)]
struct Node {
    elem: usize,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, value: usize) {
        let new_node = Link::More(Box::new(Node {
            elem: value,
            next: mem::replace(&mut self.head, Link::Empty),
        }));
        self.head = new_node;
    }
    pub fn pop(&mut self) -> Option<usize> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            Link::Empty => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);
        l.push(10);
        assert_eq!(l.pop(), Some(10));
        l.push(10);
        l.push(20);
        l.push(30);
        assert_eq!(l.pop(), Some(30));
        assert_eq!(l.pop(), Some(20));
        assert_eq!(l.pop(), Some(10));
        assert_eq!(l.pop(), None);
    }
}
