use std::{cell::RefCell, rc::Rc};

pub struct ListItem<T> {
    prev: Option<ItemRef<T>>,
    data: Box<T>,
    next: Option<ItemRef<T>>,
}

type ItemRef<T> = Rc<RefCell<ListItem<T>>>;

pub struct DoublyLinkedList<T> {
    head: ItemRef<T>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            prev: None,
            data: Box::new(data),
            next: None,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new(data: T) -> Self {
        DoublyLinkedList {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }

    pub fn append(&mut self, data: T) {
        let tail = Self::find_tail(self.head.clone());
        let new_item = Rc::new(RefCell::new(ListItem::new(data)));
        new_item.borrow_mut().prev = Some(tail.clone());
        tail.borrow_mut().next = Some(new_item);
    }

    pub fn head(&self) -> ItemRef<T> {
        self.head.clone()
    }

    pub fn tail(&self) -> ItemRef<T> {
        Self::find_tail(self.head())
    }

    pub fn find_tail(item: ItemRef<T>) -> ItemRef<T> {
        if let Some(next) = &item.borrow().next {
            Self::find_tail(next.clone())
        } else {
            item.clone()
        }
    }
}
