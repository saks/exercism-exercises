use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct SimpleLinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug, Default)]
struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            next: self.head.take(),
            value: element,
        }));

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.len -= 1;
            let node = *node;
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.value)
    }
}

impl<T: Clone + Debug> SimpleLinkedList<T> {
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut result = Self::new();

        while let Some(next) = self.pop() {
            result.push(next);
        }

        result
    }
}


impl<'a, T: Clone + Debug> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = Self::new();

        for i in item.into_iter() {
            list.push(i.clone());
        }

        list
    }
}

impl<T: Debug> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result: Vec<T> = self.into_iter().collect();
        result.reverse();
        result
    }
}

pub struct IntoIter<T: Debug>(SimpleLinkedList<T>);

impl<T: Debug> SimpleLinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Debug> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
