use core::Device;


pub struct DiscoveryInfo {
    pub name: String,
    pub definition: String
}


#[allow(async_fn_in_trait)]
pub trait DeviceDiscovery<T: DeviceDiscovery> {
    async fn discover() -> Vec<T>;
}

#[allow(async_fn_in_trait)]
pub trait Driver<D: Device> : DeviceDiscovery {

    //async fn pair(&self, discovery: &I) -> Result<(), Box<dyn std::error::Error>>;

    //async fn create(info: I) -> D;
}

