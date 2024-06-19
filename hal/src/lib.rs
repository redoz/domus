pub trait DeviceInfo {
    fn get_name(&self) -> &str;
}

pub trait Driver<I: DeviceInfo, D: Device> {
    async fn discover(&self) -> Vec<I>;

    async fn create(info: I) -> D;
}

pub trait Device {
    async fn init(&self) -> Result<(), String>;
}
