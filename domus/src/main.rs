use driver::{AqaraFP2, HueLightbulb, AqaraTemperature, SamsungOven, 
             SamsungTV, WithingsScale, SomfyBlinds, KefLS60, AqaraMiniSwitchT1, 
             AqaraP2DoorSensor, EveMotionBlinds};

fn main() {
    let apartment = space! {
        name: "Apartment",
        tags: ["home", "smart"],
        entrance_hallway: {
            name: "Entrance Hallway",
            tags: ["entrance"],
            door_sensor: {
                name: "Entrance Door Sensor",
                driver: AqaraP2DoorSensor,
                id: "0x00158d0001d82999",
                tags: ["security"],
            },
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d1",
                tags: ["lighting"],
            },
        },
        main_living_area: {
            name: "Main Living Area",
            tags: ["common area"],
            kitchen: {
                name: "Kitchen",
                tags: ["cooking"],
                ceiling_light: {
                    name: "Ceiling Light",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d2",
                    tags: ["lighting"],
                },
                temperature_sensor: {
                    name: "Temperature Sensor",
                    driver: AqaraTemperature,
                    id: "0x00158d0001d82998",
                    tags: ["climate"],
                },
                smart_oven: {
                    name: "Smart Oven",
                    driver: SamsungOven,
                    id: "192.168.1.50",
                    tags: ["appliance"],
                },
                curtains: {
                    name: "Kitchen Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e1",
                    tags: ["window"],
                },
            },
            dining_area: {
                name: "Dining Area",
                tags: ["dining"],
                pendant_light: {
                    name: "Pendant Light",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d3",
                    tags: ["lighting"],
                },
            },
            living_room: {
                name: "Living Room",
                tags: ["relaxation"],
                smart_tv: {
                    name: "Smart TV",
                    driver: SamsungTV,
                    id: "192.168.1.100",
                    tags: ["entertainment"],
                },
                floor_lamp: {
                    name: "Floor Lamp",
                    driver: HueLightbulb,
                    id: "0x00124b0022a9e5d4",
                    tags: ["lighting"],
                },
                speakers: {
                    name: "Kef LS60 Speakers",
                    driver: KefLS60,
                    id: "192.168.1.101",
                    tags: ["audio"],
                },
                curtains: {
                    name: "Living Room Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e2",
                    tags: ["window"],
                },
                balcony_curtains: {
                    name: "Balcony Door Curtains",
                    driver: EveMotionBlinds,
                    id: "0x00124b0022a9e5e3",
                    tags: ["window"],
                },
            },
        },
        office: {
            name: "Office",
            tags: ["work"],
            motion_sensor: {
                name: "Motion Sensor",
                driver: AqaraFP2,
                id: "0x00158d0001d82996",
                tags: ["security"],
            },
            desk_lamp: {
                name: "Desk Lamp",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d5",
                tags: ["lighting"],
            },
            computer: {
                name: "Computer",
                driver: GenericComputer,
                id: "00:1B:44:11:3A:B7",
                tags: ["computer"],
            },
            curtains: {
                name: "Office Curtains",
                driver: EveMotionBlinds,
                id: "0x00124b0022a9e5e4",
                tags: ["window"],
            },
        },
        bedroom_hallway: {
            name: "Bedroom Hallway",
            tags: ["bedroom"],
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d6",
                tags: ["lighting"],
            },
        },
        wc: {
            name: "WC",
            tags: ["bathroom"],
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d7",
                tags: ["lighting"],
            },
        },
        bathroom: {
            name: "Bathroom",
            tags: ["bathroom"],
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d8",
                tags: ["lighting"],
            },
        },
        bedroom: {
            name: "Bedroom",
            tags: ["bedroom"],
            ceiling_light: {
                name: "Ceiling Light",
                driver: HueLightbulb,
                id: "0x00124b0022a9e5d9",
                tags: ["lighting"],
            },
            smart_blinds: {
                name: "Smart Blinds",
                driver: SomfyBlinds,
                id: "0x00124b0022a9e5db",
                tags: ["window"],
            },
            smart_scale: {
                name: "Smart Scale",
                driver: WithingsScale,
                id: "192.168.1.150",
                tags: ["health"],
            },
            left_bedside_switch: {
                name: "Left Bedside Switch",
                driver: AqaraMiniSwitchT1,
                id: "0x00158d0001d82990",
                tags: ["lighting"],
            },
            right_bedside_switch: {
                name: "Right Bedside Switch",
                driver: AqaraMiniSwitchT1,
                id: "0x00158d0001d82991",
                tags: ["lighting"],
            },
            curtains: {
                name: "Bedroom Curtains",
                driver: EveMotionBlinds,
                id: "0x00124b0022a9e5e5",
                tags: ["window"],
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
}
