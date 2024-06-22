
#[allow(async_fn_in_trait)]
pub trait LifeCycle {
    async fn init(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
}