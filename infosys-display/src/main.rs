mod bb;
mod db;
mod util;

extern crate config;

use config::Config;

use db::db_init;
use db::retrieve_strings_for_message_id;

use util::tuple_to_bytestring;
use util::get_naivetime_now;
use util::convert_to_vec8;

use bb::START_PACKET;
use bb::END_PACKET;

use std::fs::File;
use std::io::Write;
//use std::error::Error;

fn main() {
    let settings = Config::builder().add_source(config::File::with_name("settings")).build().unwrap();
    let mut con = db_init(&settings);
    let now = get_naivetime_now();

    let mut sign_input: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let tour_mode : bool = false;


    // Get the most recent message to present
    for row in &con.query("SELECT message_id FROM schedule
        WHERE timeslot<=$1
        ORDER BY timeslot
        DESC LIMIT 1",
    &[&now]).unwrap() {
        let rowid: i32 = row.get(0);
        let result: Vec<(String, String)> = retrieve_strings_for_message_id(&mut con, rowid);
        println!("Message Id: {}, Contents:{:?}", rowid, result);
        //println!("Print result pls {:?}", result);
        if tour_mode == true { 
            let tour = String::from("MODE_STANDARD_ROLL_LEFT");
            let tour2 = String::from("Welcome to CSH!");
            let tour3 = String::from("SPECIAL_CHERRY_BOMB");
            let tour4 = String::from("Est. 1976");
            sign_input.push(tuple_to_bytestring((tour, tour2) ));
            sign_input.push(tuple_to_bytestring((tour3, tour4))); 
            let tour5 = String::from("random");
            let tour6 = String::from("Welcome to CSH!");
            let tour7 = String::from("MODE_SPECIAL_FIREWORKS");
            let tour8 = String::from("Est. 1976");
            sign_input.push(tuple_to_bytestring((tour5, tour6) ));
            sign_input.push(tuple_to_bytestring((tour7, tour8))); 
        } else {
            for res in result {
                sign_input.push(tuple_to_bytestring(res));
            
            }
        }
    }


    // Get this shit out onto the sign
    let mut file = match File::create(settings.get_string("sign_path").unwrap()) {
        Err(why) => panic!("couldn't create : {}\nDo you not have permissions?", why),
        Ok(file) => file,
    };

    file.write_all(START_PACKET).unwrap();
    file.write_all(convert_to_vec8(sign_input).as_slice()).unwrap();
    file.write_all(END_PACKET).unwrap();
    file.flush().unwrap();

}
