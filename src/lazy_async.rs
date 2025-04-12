use crate::common::Error;
use ::std::sync::Arc;
// TODO @mverleg: how to make somewhat sure there is no deadlock if the creator of ALazy doesn't put a value in?

// TODO: this could probably be more efficient by using the fact that we can only go from empty to full and never back, e.g. double checked atomic
pub struct ALazy<T> {
    value: Arc<OnceLock<Result<T, Error>>>,
}

impl <T> ALazy<T> {
    // It seems nice to pass the initializer into the factory method, since that nicely
    // models that it should happen no more and no less than one time. However, we should
    // put the ALazy into a container before it's finished constructing.
    pub async fn new_empty() -> Self {
        ALazy { value: Arc::new(OnceLock::new()) }
    }

    pub async fn get(&self) -> &Result<T, Error> {
        self.value.get().await
    }

    // TODO for now no async `f` arg, to avoid deadlocks
    pub async fn map<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, Error> {
        f(&*self.value.lock().await)
    }
}
