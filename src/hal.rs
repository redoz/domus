mod hal;

trait DeviceInfo {
    fn get_name(&self) -> &str;
}

trait Driver<I: DeviceInfo, D: Device> {
    async fn discover(&self) -> Vec<I>;

    async fn create(info: I) -> D;
}

trait Device {
    async fn init(&self) -> Result<(), String>;
}
