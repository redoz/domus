mod aqara_fp2;

use crate::aqara_fp2::AqaraFP2Driver;

#[async_std::main]
async fn main() {
 
    let driver = AqaraFP2Driver::new();

    let discovery_result = driver.discover().await;

    println!("Hello, world!");
}
