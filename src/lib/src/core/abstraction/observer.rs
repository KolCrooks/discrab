use crate::Context;

use super::abstraction_traits::{CommandArg, InternalEventHandler};

/// This struct can be subscribed to, and when it is notified, it will call the subscribers
pub struct Observable<'a, T: Clone + CommandArg> {
    /// The subscribers to the observable
    pub subscribers: Vec<&'a (dyn InternalEventHandler<T>)>,
}

impl<'a, T: Clone + CommandArg> Observable<'a, T> {
    /// Creates a new observable
    pub fn new() -> Self {
        Observable {
            subscribers: Vec::new(),
        }
    }

    /// Notifies all subscribers with given data
    pub async fn notify(&self, ctx: Context, data: T) {
        for listener in &self.subscribers {
            listener.handler(ctx.clone(), data.clone());
        }
    }

    /// Subscribes a listener to the observable
    pub fn subscribe(&mut self, listener: &'a dyn InternalEventHandler<T>) {
        self.subscribers.push(listener);
    }
}

impl<'a, T: Clone + CommandArg> Default for Observable<'a, T> {
    fn default() -> Self {
        Observable::new()
    }
}
