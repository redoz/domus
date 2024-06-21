#![allow(async_fn_in_trait)]

pub trait DiscoveryInfo {
    fn get_name(&self) -> &str;
    fn get_id(&self) -> &str;
}

pub trait Driver<I: DiscoveryInfo, D: Device> {
    async fn discover(&self) -> Vec<I>;

    async fn create(info: I) -> D;
}

pub trait Device {
    async fn init(&self) -> Result<(), String>;
}
