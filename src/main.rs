extern crate rusqlite;
extern crate chrono;
extern crate dotenv;
extern crate uuid;
extern crate libc;

use libc::c_char;
use std::ffi::{CString, CStr};
use rusqlite::Connection;
use chrono::prelude::*;
use dotenv::dotenv;
use std::env;
use std::path::Path;
use uuid::Uuid;
use std::time::Duration;
use std::thread;
use std::boxed::{Box}

const FREQUENCY: i32 = 91; 
const NODEID: i32 = 4;
const TONODEID: i32 = 2;
const NETWORKID: i32 = 0;
const TXPOWER: u8 = 31;

#[repr(C)]
struct Payload {
    uptime: u64,
    humidity: f64,
    temperature: f64,
    moisture: i32,
}

#[link(name = "rfm69", kind = "static")]
extern {
    fn wiringPiSetup();
    fn rfm69_receive();
    fn rfm69_getDataLen() -> c_char;
    fn rfm69_getData(data: *const c_char);
    fn rfm69_getDataPointer(payload: *mut Payload);
    fn rfm69_initialize(freq: i32, node_id:i32, network_id:i32, interrupt_pin: i32);
    fn rfm69_encrypt(password: &CStr);
    fn rfm69_setPowerLevel(power: c_char); // Max Power
    fn rfm69_setPromiscuous(enable: c_char);
}

#[derive(Debug)]
struct Sample {
    uuid: Uuid,
    date_time: DateTime<Utc>,
    moisture: f64,
    humidity: f64,
    temperature: f64
}



fn main() {
    dotenv().ok();
    // The statements here will be executed when the compiled binary is called
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_path = Path::new(&database_url);
    if !db_path.exists() {
        panic!("{:?} does not exist!", db_path);
    }

    println!("Opening DB at: {:?}", db_path);
    let conn = Connection::open(db_path).unwrap();
    println!("Connection: {:?}", conn);
    let CRYPTPASS = CString::new("TOPSECRETPASSWRD").unwrap();


    unsafe { 
        wiringPiSetup();
        rfm69_initialize(FREQUENCY, NODEID, NETWORKID, -1);
        rfm69_encrypt(CRYPTPASS.as_c_str());
        rfm69_setPowerLevel(TXPOWER); // Max Power
        rfm69_setPromiscuous(1);
        rfm69_getDataPointer();
    }

    let me = Sample {
        uuid: Uuid::new_v4(),
        date_time: Utc::now(),
        moisture: 3.555,
        humidity: 123.213,
        temperature: 123.23
    };
    
    
    println!("Start loop!");
    loop {
      let datalen: c_char;
      unsafe {
        println!("rfm69_receive");
        rfm69_receive();
        datalen = rfm69_getDataLen();
        println!("Datalen: {:?}", datalen);
      }
      if datalen > 0 {
        let split_str = get_sample_data();
        let moisture = split_str[5].parse::<f64>();
        let humidity = split_str[1].trim_matches('%').parse::<f64>();
        let temperature = split_str[3].parse::<f64>();

        if moisture.is_ok() && humidity.is_ok() && temperature.is_ok() {
            let me = Sample {
                uuid: Uuid::new_v4(),
                date_time: Utc::now(),
                moisture: moisture.unwrap(),
                humidity: humidity.unwrap(),
                temperature: temperature.unwrap()
            };

            let slice = &me.uuid.as_bytes().to_vec();
            println!("{:?}", me);
            conn.execute("INSERT INTO samples (uuid, DateTime, Moisture, Humidity, Temperature)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
                  &[slice, &me.date_time.to_string(), &me.moisture, &me.humidity, &me.temperature]).unwrap();
        }

        thread::sleep(Duration::from_secs(300)); // sleep 5min
      }
    }
}

fn get_sample_data() -> Vec<&'static str> {
  unsafe {
      let mut payload = Box::new(Payload {uptime: 0, humidity: 0.0, temperature: 0.0, moisture: 0});
      rfm69_getDataPointer(&mut *payload);
      println!("uptime: {:?}", payload.uptime);
      println!("humidity: {:?}", payload.humidity);
      let received_vec = vec![0; 86];
      let c_string = CString::from_vec_unchecked(received_vec);
      rfm69_getData(c_string.as_ptr());
      let slice = CStr::from_ptr(c_string.as_ptr());
        println!("{:?}", slice.to_str().unwrap().split(",").collect::<Vec<&str>>());
  return slice.to_str().unwrap().split(",").collect::<Vec<&str>>();
  }
}
