use clap::{Arg, Command};
use core::{Driver, DiscoveryInfo};
use driver::AqaraFP2Driver; // Import both the struct and the trait
use std::process::exit; // Added this line to import the exit function

#[tokio::main]
async fn main() {
    env_logger::init();


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
        .subcommand(
            Command::new("pair")
                .arg(
                    Arg::new("driver")
                        .short('d')
                        .long("driver")
                        .value_name("NAME")
                        .help("Driver to use")
                        .value_parser(["aqarafp2"])
                        .required(true),
                )
                .arg(
                    Arg::new("id")
                        .long("id")
                        .value_name("NAME")
                        .help("Device identifier")
                        .required(true),
                )                
                .arg(Arg::new("debug").long("debug").help("Turn on debugging")),
        )        
        .get_matches();

    if let Some(subcommand) = matches.subcommand() {
        match subcommand {
            ("scan", cmd) => {

                let driver = cmd.get_one::<String>("driver").unwrap();

                match driver.as_str() {
                    "aqarafp2" => {
                        println!("Scanning for Aqara FP2 devices");
                        let driver = AqaraFP2Driver::new();
                        let discoveries = driver.discover().await;
                        
                        if discoveries.len() == 0 {
                            println!("No devices found");
                            exit(0);
                        }

                        for discovery in discoveries {
                            println!("Discovered Aqara FP2 device: {} (id: {})", discovery.name(), discovery.id());
                        }

                        
                    },
                    _ => panic!("Unknown driver: {}", driver)
                }
            },
            ("pair", cmd) => {
                let driver = cmd.get_one::<String>("driver").unwrap();
                let device_id = cmd.get_one::<String>("id").unwrap();

                match driver.as_str() {
                    "aqarafp2" => {
                        println!("Pairing Aqara FP2 device with id {}", device_id);
                        let driver = AqaraFP2Driver::new();
                        let discoveries = driver.discover().await;
                        let discovery = discoveries.iter().filter(|d| d.id() == device_id).next();
                        if let None = discovery {
                            println!("Could not find Aqara FP2 device with id: {}", device_id);
                            exit(1);
                        }
                        println!("Device found, attempting to pair...");
                        let _pairing = driver.pair(discovery.unwrap()).await;
                    },
                    _ => panic!("Unknown driver: {}", driver)
                }
            },
            ("drivers", _) => {
                println!("Listing all available drivers");
            }
            _ => {}
        }
    }
}
