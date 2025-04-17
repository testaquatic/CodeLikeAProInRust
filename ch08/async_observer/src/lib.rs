use std::{
    pin::Pin,
    sync::{Arc, Weak},
};

pub trait Observer: Send + Sync {
    type Subject;
    type Output;
    fn observe<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>>;
}

pub trait Observable {
    type Observer;
    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self, Output = ()>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: Vec::new(),
            state: state.to_string(),
        }
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self, Output = ()>>;
    fn update<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> {
        let observers = self
            .observers
            .iter()
            .flat_map(|o| o.upgrade())
            .collect::<Vec<_>>();

        Box::pin(async move {
            futures::future::join_all(observers.iter().map(|o| o.observe(self))).await;
        })
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}

#[derive(Clone)]
pub struct MyObserver(String);

impl MyObserver {
    pub fn new(state: &str) -> Arc<Self> {
        Arc::new(Self(state.to_string()))
    }
}

impl Observer for MyObserver {
    type Subject = Subject;
    type Output = ();
    fn observe<'a>(
        &'a self,
        subject: &'a Self::Subject,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>> {
        Box::pin(async move {
            println!(
                "observed subject with state=\"{}\" in {}",
                &subject.state, self.0
            );
        })
    }
}
