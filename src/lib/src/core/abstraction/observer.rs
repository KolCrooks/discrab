use std::{sync::Arc, panic::{self, RefUnwindSafe, UnwindSafe}};

use crate::{Context, util::logger::print_debug};

use super::traits::{CommandArg, __InternalEventHandler};

/// This struct can be subscribed to, and when it is notified, it will call the subscribers
pub struct Observable<T: Clone + CommandArg + UnwindSafe + RefUnwindSafe> {
    /// The subscribers to the observable
    pub subscribers: Vec<Arc<dyn __InternalEventHandler<T>>>,
}

impl<T: Clone + CommandArg + UnwindSafe + RefUnwindSafe> Observable<T> {
    /// Creates a new observable
    pub fn new() -> Self {
        Observable {
            subscribers: Vec::new(),
        }
    }

    /// Notifies all subscribers with given data
    pub fn notify(&self, ctx: Context, data: T) {
        for listener in &self.subscribers {
            panic::catch_unwind(|| {
                listener.handler(ctx.clone(), data.clone());
            }).unwrap_or_else(|t| {
                println!("Unhandled panic in observable: {:?}", t);
            });
        }
    }

    /// Subscribes a listener to the observable
    pub fn subscribe(&mut self, listener: Arc<dyn __InternalEventHandler<T>>) {
        self.subscribers.push(listener);
    }
}

impl<T: Clone + CommandArg + UnwindSafe + RefUnwindSafe> Default for Observable<T> {
    fn default() -> Self {
        Observable::new()
    }
}
