use crate::common::Error;
use ::futures::lock::Mutex as AsyncMutex;
use ::futures::lock::MutexGuard;
use ::std::sync::Arc;

// TODO @mverleg: how to make somewhat sure there is no deadlock if the creator of ALazy doesn't put a value in?

// TODO: this could probably be more efficient by using the fact that we can only go from empty to full and never back, e.g. double checked atomic
pub struct ALazy<T> {
    value: Arc<AsyncMutex<Option<T>>>,
}

impl <T> ALazy<T> {
    pub async fn new_empty() -> Result<Self, Error> {
        ALazy { value: Arc::new(AsyncMutex::new(None)) }.await
    }

    pub async fn get(&self) -> MutexGuard<'_, Result<T, Error>> {
        self.value.lock().await
    }

    // TODO for now no async `f` arg, to avoid deadlocks
    pub async fn map<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, Error> {
        f(&*self.value.lock().await)
    }
}
