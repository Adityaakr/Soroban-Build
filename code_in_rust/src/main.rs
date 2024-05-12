fn main() {
    //variables
    //let -> immutable
    //let -> mutable

    //basic data Types -> Rust in any Language

    let x: i32 = 16;
    //to print this we need to write macro => println!
    println!("{}", x); //first version

    let z: String = String::from("Hello Soroban"); //mutable string
    let y: &str = "Hello Stellar!"; //immutable string

    println!("{y}");
    println!("{z}");

    // pub fn event (name: String) {
    //     let name: String = String::from("Soroban");
    //     println!("{name}")
    // }
  
    const e: EventForKids = EventForKids {
        name: String::from("KidsCo"), // String literal
        date: String::from( "04.03.2005"), // String literal
        number_of_participants: 1000,
        place: String::from( "NY, USA"), // String literal
    };
    

    //add enum here

    


     
} fn main

//compiling many items in one class
struct EventForKids {
    name: String,
    date: String,
    number_of_participants: u32,
    place: String,
}

enum {
    NotEvent,
    CancelledEvent,
    EventType
}