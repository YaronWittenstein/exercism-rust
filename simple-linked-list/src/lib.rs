pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        match self.head {
            None => 0,
            Some(_) => {
                let mut current: &Option<_> = &self.head;
                let mut length = 0;

                while current.is_some() {
                    current = &current.as_ref().unwrap().next;
                    length += 1;
                }

                length
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            data: element,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(_) => {
                let old_head: Node<_> = *std::mem::replace(&mut self.head, None).unwrap();

                self.head = old_head.next;

                Some(old_head.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref head) => Some(&head.data),
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut vec = SimpleLinkedList::<T>::new();
        let mut current: &Option<_> = &self.head;

        while current.is_some() {
            let node = &current.as_ref().unwrap();

            vec.push(node.data.clone());

            current = &node.next;
        }

        vec
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut vec = SimpleLinkedList::<T>::new();

        for v in item {
            vec.push(v.clone());
        }

        vec
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::<T>::new();

        let mut head = self.head;

        while head.is_some() {
            let old_head: Option<_> = head.take();
            let unwrapped: Node<T> = *old_head.unwrap();
            let new_head: Option<_> = unwrapped.next;

            let v = unwrapped.data;
            vec.insert(0, v);

            head = new_head;
        }

        vec
    }
}
