pub struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: ListItem<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }

    pub fn next(&self) -> Option<&Self> {
        if let Some(ref next) = self.next {
            Some(next)
        } else {
            None
        }
    }

    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() {
            self.next.as_mut().unwrap().mut_tail()
        } else {
            self
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data),
        }
    }

    pub fn append(&mut self, data: T) {
        let tail = self.head.mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
    }

    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }
}
