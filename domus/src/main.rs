#[macro_use]
mod domus_macro;
use paste::paste;

use core::LifeCycle;
use core::Space;




/*
pub struct SubSpaceIterator<'a, S> {
    owner: &'a S,
    //space: &'a dyn Space,
    index: usize,
}
 */

use driver::AqaraFP2;





#[async_std::main]
async fn main() {
    env_logger::init();

    let apartment = domus! {
        name: "Apartment",
        office: Space {
            name: "Office",
            motion_sensor: AqaraFP2 { 
                name: "Motion sensor"
            },
        },
    };

    
    if let Err(error) = apartment.init().await {
        log::error!("Error initializing: {:?}", error);
    } else {
        // do some kind of waiting for exit signal here
    }
    
/*
    let apartment = domus! {
        name: "Apartment",
        entrance_hallway: {
            name: "Entrance Hallway",
            door_sensor: AqaraP2DoorSensor {
                name: "Entrance Door Sensor",
                id: "0x00158d0001d82999",
            },
            ceiling_light: HueLightbulb {
                name: "Ceiling Light",
                id: "0x00124b0022a9e5d1",
            },
        },
        main_living_area: {
            name: "Main Living Area",
            kitchen: {
                name: "Kitchen",
                ceiling_light: HueLightbulb {
                    name: "Ceiling Light",
                    id: "0x00124b0022a9e5d2",
                },
                temperature_sensor: AqaraTemperature {
                    name: "Temperature Sensor",
                    id: "0x00158d0001d82998",
                },
                smart_oven: SamsungOven {
                    name: "Smart Oven",
                    id: "192.168.1.50",
                },
                curtains: EveMotionBlinds {
                    name: "Kitchen Curtains",
                    id: "0x00124b0022a9e5e1",
                },
            },
            dining_area: {
                name: "Dining Area",
                pendant_light: HueLightbulb {
                    name: "Pendant Light",
                    id: "0x00124b0022a9e5d3",
                },
            },
            living_room: {
                name: "Living Room",
                smart_tv: SamsungTV {
                    name: "Smart TV",
                    id: "192.168.1.100",
                },
                floor_lamp: HueLightbulb {
                    name: "Floor Lamp",
                    id: "0x00124b0022a9e5d4",
                },
                speakers: KefLS60 {
                    name: "Kef LS60 Speakers",
                    id: "192.168.1.101",
                },
                curtains: EveMotionBlinds {
                    name: "Living Room Curtains",
                    id: "0x00124b0022a9e5e2",
                },
                balcony_curtains: EveMotionBlinds {
                    name: "Balcony Door Curtains",
                    id: "0x00124b0022a9e5e3",
                },
            },
        },
        office: {
            name: "Office",
            motion_sensor: AqaraFP2 {
                name: "Motion Sensor",
                id: "0x00158d0001d82996",
            },
            desk_lamp: HueLightbulb {
                name: "Desk Lamp",
                id: "0x00124b0022a9e5d5",
            },
            computer: GenericComputer {
                name: "Computer",
                id: "00:1B:44:11:3A:B7",
            },
            curtains: EveMotionBlinds {
                name: "Office Curtains",
                id: "0x00124b0022a9e5e4",
            },
        },
        bedroom_hallway: {
            name: "Bedroom Hallway",
            ceiling_light: HueLightbulb {
                name: "Ceiling Light",
                id: "0x00124b0022a9e5d6",
            },
        },
        wc: {
            name: "WC",
            ceiling_light: HueLightbulb {
                name: "Ceiling Light",
                id: "0x00124b0022a9e5d7",
            },
        },
        bathroom: {
            name: "Bathroom",
            ceiling_light: HueLightbulb {
                name: "Ceiling Light",
                id: "0x00124b0022a9e5d8",
            },
        },
        bedroom: {
            name: "Bedroom",
            ceiling_light: HueLightbulb {
                name: "Ceiling Light",
                id: "0x00124b0022a9e5d9",
            },
            smart_blinds: SomfyBlinds {
                name: "Smart Blinds",
                id: "0x00124b0022a9e5db",
            },
            smart_scale: WithingsScale {
                name: "Smart Scale",
                id: "192.168.1.150",
            },
            left_bedside_switch: AqaraMiniSwitchT1 {
                name: "Left Bedside Switch",
                id: "0x00158d0001d82990",
            },
            right_bedside_switch: AqaraMiniSwitchT1 {
                name: "Right Bedside Switch",
                id: "0x00158d0001d82991",
            },
            curtains: EveMotionBlinds {
                name: "Bedroom Curtains",
                id: "0x00124b0022a9e5e5",
            },
        },
    };
 */
    
    
    /*
    let apartment = domus! {
        name: "Apartment",
        entrance_hallway: {
            name: "Entrance Hallway",
            door_sensor: {
                name: "Entrance Door Sensor",
                driver: AqaraP2DoorSensor,
                id: "0x00158d0001d82999",
            },
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d1",
            },
        },
        main_living_area: {
            name: "Main Living Area",
            kitchen: {
                name: "Kitchen",
                ceiling_light: {
                    name: "Ceiling Light",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d2",
                },
                temperature_sensor: {
                    name: "Temperature Sensor",
                    driver: AqaraTemperature,
                    id: "0x00158d0001d82998",
                },
                smart_oven: {
                    name: "Smart Oven",
                    driver: SamsungOven,
                    id: "192.168.1.50",
                },
                curtains: {
                    name: "Kitchen Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e1",
                },
            },
            dining_area: {
                name: "Dining Area",
                pendant_light: {
                    name: "Pendant Light",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d3",
                },
            },
            living_room: {
                name: "Living Room",
                smart_tv: {
                    name: "Smart TV",
                    driver: SamsungTV,
                    id: "192.168.1.100",
                },
                floor_lamp: {
                    name: "Floor Lamp",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d4",
                },
                speakers: {
                    name: "Kef LS60 Speakers",
                    driver: KefLS60,
                    id: "192.168.1.101",
                },
                curtains: {
                    name: "Living Room Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e2",
                },
                balcony_curtains: {
                    name: "Balcony Door Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e3",
                },
            },
        },
        office: {
            name: "Office",
            motion_sensor: {
                name: "Motion Sensor",
                driver: AqaraFP2,
                id: "0x00158d0001d82996",
            },
            desk_lamp: {
                name: "Desk Lamp",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d5",
            },
            computer: {
                name: "Computer",
                driver: GenericComputer,
                id: "00:1B:44:11:3A:B7",
            },
            curtains: {
                name: "Office Curtains",
                driver: EveMotionBlinds,
                id: "0x00124b0022a9e5e4",
            },
        },
        bedroom_hallway: {
            name: "Bedroom Hallway",
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d6",
            },
        },
        wc: {
            name: "WC",
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d7",
            },
        },
        bathroom: {
            name: "Bathroom",
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d8",
            },
        },
        bedroom: {
            name: "Bedroom",
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d9",
            },
            smart_blinds: {
                name: "Smart Blinds",
                driver: SomfyBlinds,
                id: "0x00124b0022a9e5db",
            },
            smart_scale: {
                name: "Smart Scale",
                driver: WithingsScale,
                id: "192.168.1.150",
            },
            left_bedside_switch: {
                name: "Left Bedside Switch",
                driver: AqaraMiniSwitchT1,
                id: "0x00158d0001d82990",
            },
            right_bedside_switch: {
                name: "Right Bedside Switch",
                driver: AqaraMiniSwitchT1,
                id: "0x00158d0001d82991",
            },
            curtains: {
                name: "Bedroom Curtains",
                driver: EveMotionBlinds,
                id: "0x00124b0022a9e5e5",
            },
        },
    };

    println!("Apartment setup complete!");
    
    // You can still reference devices like this:
    let entrance_door = &apartment.entrance_hallway.door_sensor;
    let kitchen_temp = &apartment.main_living_area.kitchen.temperature_sensor;
    let office_motion = &apartment.office.motion_sensor;
    let living_room_speakers = &apartment.main_living_area.living_room.speakers;
    let bedroom_scale = &apartment.bedroom.smart_scale;
    let kitchen_curtains = &apartment.main_living_area.kitchen.curtains;
    let balcony_curtains = &apartment.main_living_area.living_room.balcony_curtains;
    */
}
