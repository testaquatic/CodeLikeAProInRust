use std::borrow::Cow;

/// `Cow`는 `Clone`트레이트의 동작에 의존한다.
/// [러스트 문서](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html)를 찾아보니
/// ```ignore
/// impl<T> ToOwned for T
/// where
///    T: Clone,
/// ```
/// 이런 구현이 있다.
#[derive(Clone)]
pub struct ListItem<T>
where
    T: Clone,
{
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

impl<T> ListItem<T>
where
    T: Clone,
{
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }

    #[allow(dead_code)]
    fn next(&self) -> Option<&Self> {
        self.next.as_ref().map(|x| x as &Self)
    }

    fn mut_tail(&mut self) -> &mut Self {
        match self.next {
            Some(ref mut tail) => tail.mut_tail(),
            None => self,
        }
    }

    /// 데이터의 레퍼런스를 반환한다.
    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Clone)]
pub struct SinglyLinkedList<'a, T>
where
    T: Clone,
{
    head: Cow<'a, ListItem<T>>,
}

impl<'a, T> SinglyLinkedList<'a, T>
where
    T: Clone,
{
    pub fn new(data: T) -> Self {
        Self {
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
