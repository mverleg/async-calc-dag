use ::futures::lock::Mutex as AsyncMutex;
use ::futures::lock::MutexGuard;

pub struct ALazy<T> {
    value: AsyncMutex<T>,
}

impl <T> ALazy<T> {
    pub async fn get(&self) -> MutexGuard<'_, T> {
        self.value.lock().await
    }

    // TODO for now no async `f` arg, to avoid deadlocks
    pub async fn map<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(&*self.value.lock().await)
    }
}
