impl Space {
    fn new(name: &str) -> Self {
        Space {
            name: name.to_string(),
        }
    }
}

struct Space {
    name: String,
}

struct Desk {
    
}

struct Office {
    ceiling_light: Device,

    desk: Desk
}

struct Apartment {
    office: Office,
}


fn main() {

    let apartment = Space::new("Apartment");
    println!("Hello, world!");
}
