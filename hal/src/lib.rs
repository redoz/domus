use core::LifeCycle;


pub trait DiscoveryInfo {
    fn get_name(&self) -> &str;
    fn get_id(&self) -> &str;
}

#[allow(async_fn_in_trait)]
pub trait Driver<I: DiscoveryInfo, D: Device> {
    async fn discover(&self) -> Vec<I>;
    async fn pair(&self, discovery: I) -> Result<(), Box<dyn std::error::Error>>;

    async fn create(info: I) -> D;
}

pub trait Device : LifeCycle {

}
