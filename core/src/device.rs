use std::fmt::Display;
use crate::LifeCycle;

pub trait DiscoveryInfo {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
}

pub trait DeviceProperties : Display {
}

pub trait Device : LifeCycle {
}



#[allow(async_fn_in_trait)]
pub trait Driver<I: DiscoveryInfo, P: DeviceProperties, D: Device> {
    async fn discover(&self) -> Vec<I>;
    async fn pair(&self, discovery: &I) -> Result<P, Box<dyn std::error::Error>>;

    //async fn create(info: I) -> D;
}