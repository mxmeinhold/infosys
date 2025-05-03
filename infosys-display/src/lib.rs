mod bb;
mod db;
mod util;

use config::Config;

use db::db_init;
use db::retrieve_strings_for_message_id;

use util::convert_to_vec8;
use util::get_naivetime_now;
use util::tuple_to_bytestring;

use bb::END_PACKET;
use bb::START_PACKET;

use std::fs::File;
use std::io::Write;
//use std::error::Error;
use chrono::Local;
use std::fs::OpenOptions;

pub async fn tour_mode() {
    let mut tour = String::from("Tour Mode, ");
    tour.push_str(&Local::now().to_string());
    let path = String::from("temp.txt");
    //    let mut f = match File::create(path) {
    //            Err(why) => panic!("Couldn't create : {}\n dumbass", why),
    //            Ok(file) => file,
    //    };
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(f, "{}", tour).expect("write failed");
    //    f.flush().unwrap();

    //if let Err(e) = writeln!(f, "A new line!") {
    //    eprintln!("Couldn't write to file: {}", e);
    //}

    let settings = Config::builder()
        .add_source(config::File::with_name("infosys-display/settings"))
        .build()
        .unwrap();

    let mut sign_input: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    let tour = String::from("MODE_STANDARD_ROLL_LEFT");
    let tour2 = String::from("Welcome to CSH!");
    let tour3 = String::from("SPECIAL_CHERRY_BOMB");
    let tour4 = String::from("Est. 1976");
    sign_input.push(tuple_to_bytestring((tour.clone(), tour2.clone())));
    sign_input.push(tuple_to_bytestring((tour3.clone(), tour4.clone())));
    let tour5 = String::from("random");
    let tour6 = String::from("Welcome to CSH!");
    let tour7 = String::from("MODE_SPECIAL_FIREWORKS");
    let tour8 = String::from("Est. 1976");
    sign_input.push(tuple_to_bytestring((tour5.clone(), tour6.clone())));
    sign_input.push(tuple_to_bytestring((tour7.clone(), tour8.clone())));

    let mut file = match File::create(settings.get_string("sign_path").unwrap()) {
        Err(why) => panic!("couldn't create : {}\ndo you not have permissions?", why),
        Ok(file) => file,
    };
    // let _message_log = sign_input.clone();
    file.write_all(START_PACKET).unwrap();
    file.write_all(convert_to_vec8(sign_input).as_slice())
        .unwrap();
    file.write_all(END_PACKET).unwrap();
    writeln!(
        f,
        "{}, {}, {}, {}, {}, {}, {}, {}",
        tour, tour2, tour3, tour4, tour5, tour6, tour7, tour8
    )
    .unwrap();
    file.flush().unwrap();
} // end tour_mode

pub async fn grab_from_db() {
    let mut db = String::from("DB Mode, ");
    db.push_str(&Local::now().to_string());
    let path = String::from("temp.txt");
    //    let mut f = match File::create(path) {
    //            Err(why) => panic!("Couldn't create : {}\n dumbass", why),
    //            Ok(file) => file,
    //    };
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    let settings = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();
    let mut con = db_init(&settings);
    let now = get_naivetime_now();

    let mut sign_input: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    // Get the most recent message to present
    for row in &con
        .query(
            "SELECT message_id FROM schedule
        WHERE timeslot<=$1
        ORDER BY timeslot
        DESC LIMIT 1",
            &[&now],
        )
        .unwrap()
    {
        let rowid: i32 = row.get(0);
        let result: Vec<(String, String)> = retrieve_strings_for_message_id(&mut con, rowid);
        println!("Message Id: {}, Contents:{:?}", rowid, result);
        //println!("Print result pls {:?}", result);
        writeln!(f, "Message Id: {}, Contents:{:?}", rowid, result).unwrap();
        for res in result {
            sign_input.push(tuple_to_bytestring(res));
        }
    }

    // Get this shit out onto the sign
    let mut file = match File::create(settings.get_string("sign_path").unwrap()) {
        Err(why) => panic!("couldn't create : {}\ndo you not have permissions?", why),
        Ok(file) => file,
    };

    file.write_all(START_PACKET).unwrap();
    file.write_all(convert_to_vec8(sign_input).as_slice())
        .unwrap();
    file.write_all(END_PACKET).unwrap();
    file.flush().unwrap();
}
