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
                name: "Offic motion sensor"
            }
        }
    };

    if let Err(error) = apartment.init().await {
        log::error!("Error initializing: {:?}", error);
    } else {
        // start up the state-machines
    }

    log::info!("Shutting down...");
    if let Err(error) = apartment.dispose().await {
        log::error!("Error disposing: {:?}", error);
        std::process::exit(1);
    }


/*
    let apartment = domus! {
        name: "Apartment",
        
        entrance: Space {
            name: "Entryway",
            door_sensor: DummyDevice { 
                device_type: "Aqara P2 Door Sensor",
                name: "Apartment door"
            },
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "Entryway light"
            },
        },
        
        main_living_area: Space {
            name: "Main Living Area",
            kitchen: Space {
                name: "Kitchen",
                ceiling_light: DummyDevice {
                    device_type: "Ceiling Light",
                    name: "Kitchen light"
                },
                motion_sensor: DummyDevice {
                    device_type: "Motion Sensor",
                    name: "Kitchen motion sensor"
                },
                curtains: DummyDevice {
                    device_type: "Smart Curtains",
                    name: "Kitchen curtains"
                },
            },
            dining_area: Space {
                name: "Dining Area",
                ceiling_light: DummyDevice {
                    device_type: "Ceiling Light",
                    name: "Dining area light"
                },
                motion_sensor: DummyDevice {
                    device_type: "Motion Sensor",
                    name: "Dining area motion sensor"
                },
            },
            living_room: Space {
                name: "Living Room",
                ceiling_light: DummyDevice {
                    device_type: "Ceiling Light",
                    name: "Living room light"
                },
                motion_sensor: DummyDevice {
                    device_type: "Motion Sensor",
                    name: "Living room motion sensor"
                },
                curtains: DummyDevice {
                    device_type: "Smart Curtains",
                    name: "Living room curtains"
                },
                balcony_curtains: DummyDevice {
                    device_type: "Smart Curtains",
                    name: "Balcony door curtains"
                },
            },
        },
        
        hallway: Space {
            name: "Hallway",
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "Hallway light"
            },
            motion_sensor: DummyDevice {
                device_type: "Motion Sensor",
                name: "Hallway motion sensor"
            },
        },
        
        bedroom: Space {
            name: "Bedroom",
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "Bedroom light"
            },
            motion_sensor: DummyDevice {
                device_type: "Motion Sensor",
                name: "Bedroom motion sensor"
            },
        },
        
        wc: Space {
            name: "WC",
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "WC light"
            },
            motion_sensor: DummyDevice {
                device_type: "Motion Sensor",
                name: "WC motion sensor"
            },
        },
        
        bathroom: Space {
            name: "Bathroom",
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "Bathroom light"
            },
            motion_sensor: DummyDevice {
                device_type: "Motion Sensor",
                name: "Bathroom motion sensor"
            },
        },
        
        office: Space {
            name: "Office",
            motion_sensor: AqaraFP2 { 
                name: "Motion sensor"
            },
            ceiling_light: DummyDevice {
                device_type: "Ceiling Light",
                name: "Office light"
            },
            curtains: DummyDevice {
                device_type: "Smart Curtains",
                name: "Office curtains"
            },
        },
    };
 */
    

 
 /*
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
