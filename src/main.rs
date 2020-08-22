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

//main: Wraps the whole thing together. Using two constraint variables as my
//sentinel values for swapping multiplexer ports. Takes 10 readings for each
//port in the range of my constraints, fetching both the temperature and 
//capacitance values at a 500ms delay, and prints them both out on one line.
//
fn main() {
    let mut temp: f32;
    let mut cap: u16;

    let plant_begin: u8 = 1;
    let plant_end: u8 = 2;
    
    let mut conn = init(SERVER_URL);

    for i in plant_begin..plant_end + 1 {
        let name = plant_name(i);

        match muxer(i) {
            Ok(n) => n,
            Err(err) => println!("Error communicating with multiplexer!: {}", err),
        };  

        println!("\nSwitching to multiplex port {}: \n\n", i);

        temp = sensetemp(500);
        cap = sensecap(500);

        println!("Temperature: {}\nCapacitance: {}\n", temp, cap);
            
        println!("Writing to database...");
        match insert(&mut conn, name, temp, cap) {
           Ok(m) => m,
           Err(err) => println!("Error writing to database!!! {}", err),
        }
    }
}
