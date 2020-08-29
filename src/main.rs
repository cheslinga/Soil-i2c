mod soil_lib;
mod data;

pub use crate::soil_lib::i2conn::muxer;
pub use crate::soil_lib::stemconn::{sensetemp, sensecap};
pub use crate::data::d_mysql::{init, insert};

pub const SERVER_URL: &str = "mysql://USER:PASSWORD@localhost/DATABASE";

//plant_name: Just a small function to construct a name for the plant to insert
//into the database. I'm just using a basic numbering system, but a case statement
//could be used to give unique names to plants based on the value of i in main.
//
fn plant_name (num: u8) -> String {
    let plname: String = format!("Plant {}", num);
    return plname;
}

//read_plant: Pulls in a plant name (usually made via plant_name) and makes the
//connection to the SQL server. Creates two variables from sensing the temperature
//and capacitance, prints them, then writes them plus their name to the database
//that it connected to.
//
fn read_plant(name: String) {
    let mut conn = init(SERVER_URL);

    let temp:f32 = sensetemp(500);
    let cap:u16 = sensecap(500);

    println!("Temperature: {}\nCapacitance: {}\n", temp, cap);
            
    println!("Writing to database...");
    insert(&mut conn, name, temp, cap);
}
//run: Created this function to make things more functional and extensible, and because 
//not everybody will elect to run with a multiplexer. This just takes a range between two
//numbers, switches to that numbered multiplexer port, and sends that plant data to the 
//database.
//
fn run(plant_begin:u8, plant_end:u8) {
    for i in plant_begin..plant_end + 1 {
        muxer(i);
        read_plant(plant_name(i));
    }
}

//main: Made main super simple for extensibility and to try and use functional programming
//wherever possible. This way you could add in a scheduler crate and set the run function
//to happen at certain times, or have things that happen between runs, etc.
//
fn main() {
    run(1,3);
}
