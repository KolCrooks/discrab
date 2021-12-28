use async_trait::async_trait;

use crate::{Context, Events};

#[async_trait]
pub trait EventHandler<T> {
    const EVENT: Events;

    async fn handle(_: Context, _: T);
}
