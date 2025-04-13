use crate::common::{Error, Share};
use ::std::sync::Arc;
use ::tokio::sync::OnceCell;
// TODO @mverleg: how to make somewhat sure there is no deadlock if the creator of ALazy doesn't put a value in?

// TODO: this could probably be more efficient by using the fact that we can only go from empty to full and never back, e.g. double checked atomic
pub struct ALazy<T> {
    value: Arc<OnceCell<Result<T, Error>>>,
}

impl <T> ALazy<T> {
    // It seems nice to pass the initializer into the factory method, since that nicely
    // models that it should happen no more and no less than one time. However, we should
    // put the ALazy into a container before it's finished constructing.
    pub fn new() -> Self {
        ALazy { value: Arc::new(OnceCell::new()) }
    }

    pub async fn get<F>(&self, f: impl FnOnce() -> F) -> &Result<T, Error>
            where F: Future<Output=Result<T, Error>> {
        self.value.get_or_init(f).await
    }
}

impl <T> Share for ALazy<T> {
    fn share(&self) -> Self {
        ALazy { value: self.value.clone() }
    }
}