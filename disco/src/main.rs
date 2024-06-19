use clap::{Arg, Command};
use driver::AqaraFP2Driver; // Import the trait
use hal::{Driver, DeviceInfo};

#[async_std::main]
async fn main() {
    let matches = Command::new("disco")
        .version("1.0")
        .author("Patrik Husfloen <redoz@redoz.com>")
        .about("Device discovery tool")
        .subcommand(Command::new("drivers").about("Lists all available drivers"))
        .subcommand(
            Command::new("scan")
                .arg(
                    Arg::new("driver")
                        .short('d')
                        .long("driver")
                        .value_name("NAME")
                        .help("Driver to use for discovery"),
                )
                .arg(Arg::new("debug").long("debug").help("Turn on debugging")),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("scan", sub_m) => {
                if let Some(driver) = sub_m.get_one::<String>("driver") {
                    println!("Value for driver: {}", driver);
                }
                /* 
                if matches.contains_id("debug") {
                    println!("Debugging is on");
                }
                */

                let fp2 = AqaraFP2Driver::new();
                let result = fp2.discover().await;
                for item in result {
                    println!("Discovered item: {:?}", item.get_name());
                }
            }
            ("drivers", _) => {
                println!("Listing all available drivers");
            }
            _ => {}
        }
    }
}
