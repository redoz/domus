use clap::{Arg, Command};
use driver::AqaraFP2Driver; // Import the trait
use hal::DeviceDiscovery;



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
                        .help("Driver to use for discovery")
                        .value_parser(["aqarafp2"])
                        .required(true),
                )
                .arg(Arg::new("debug").long("debug").help("Turn on debugging")),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("scan", sub_m) => {

                let driver = sub_m.get_one::<String>("driver").unwrap();

                match driver.as_str() {
                    "aqarafp2" => {
                        let discoveries = AqaraFP2Driver::discover().await;
                        for discovery in discoveries {
                            println!("Discovered Aqara FP2 device: {}", discovery.name);
                            println!("Definition: {}", discovery.definition);
                        }
                    },
                    _ => panic!("Unknown driver: {}", driver)
                }
            }
            ("drivers", _) => {
                println!("Listing all available drivers");
            }
            _ => {}
        }
    }
}
