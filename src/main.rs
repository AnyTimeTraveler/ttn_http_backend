#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
#[macro_use]
extern crate rocket;

use std::env;

use reqwest::Client;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct UplinkData {
    app_id: String,
    dev_id: String,
    hardware_serial: String,
    port: i32,
    counter: i32,
    payload_raw: String,
    downlink_url: String,
}

#[derive(Serialize, Debug)]
struct DownlinkData {
    dev_id: String,
    port: i32,
    confirmed: bool,
    payload_raw: String,
}

static APP_ID: &str = "any_talk_traveler";
static DEV_ID: &str = "walkietalkie0";
static PROCESS_ID: &str = "ttn_http_test";


#[post("/uplink", format = "json", data = "<input>")]
fn uplink(input: Json<UplinkData>) {
    let data = base64::decode(&input.payload_raw);
    let string = String::from_utf8(data.expect("AAAA"));

    println!("Message received: {:?}", string);
    downlink(String::from(&input.dev_id), "Hello device".as_bytes()).expect("AAAAA");
}


type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn downlink(to: String, data: &[u8]) -> Result<()> {
    let client = Client::new();
    let downlink_data = DownlinkData {
        dev_id: to,
        port: 1,
        confirmed: false,
        payload_raw: base64::encode(data),
    };
    let url = format!("https://integrations.thethingsnetwork.org/ttn-eu/api/v2/down/{}/{}", APP_ID, PROCESS_ID);
    let req = client.post(&url)
        .json(&downlink_data)
        .header("User-Agent", "ttn_http_test/1.0")
        .query(&[("key", env::var("TTN_KEY").expect("Missing TTN_KEY environment variable!"))]);

    let res = req.send().await?;
    println!("TTN Status: {}", res.status());

    let body = res.bytes().await?;

    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("TTN TEXT Response: {} ", s);
    Ok(())
}


fn main() {
    rocket::ignite().mount("/", routes![uplink]).launch();
}

