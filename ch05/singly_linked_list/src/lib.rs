use std::borrow::Cow;

#[derive(Clone)]
pub struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

#[derive(Clone)]
pub struct SinglyLinkedList<'a, T>
where
    T: Clone,
{
    head: Cow<'a, ListItem<T>>,
}

impl<T> ListItem<T> {
    fn new(data: T) -> ListItem<T> {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }

    pub fn next(&self) -> Option<&ListItem<T>> {
        self.next.as_ref().map(|x| x.as_ref())
    }

    fn mut_tail(&mut self) -> &mut ListItem<T> {
        match self.next {
            Some(ref mut next) => next.mut_tail(),
            None => self,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> SinglyLinkedList<'_, T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        SinglyLinkedList {
            head: Cow::Owned(ListItem::new(data)),
        }
    }

    pub fn append(&self, data: T) -> Self {
        let mut new_list = self.clone();
        let tail = new_list.head.to_mut().mut_tail();
        tail.next = Some(Box::new(ListItem::new(data)));
        new_list
    }

    pub fn head(&self) -> &ListItem<T> {
        &self.head
    }
}
