#![feature(proc_macro_hygiene, decl_macro)]

extern crate base64;
#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::{Request, State};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Debug)]
struct UplinkData {
    app_id: String,
    dev_id: String,
    hardware_serial: String,
    port: u8,
    counter: u32,
    payload_raw: String,
}

#[derive(Serialize, Debug)]
struct DownlinkData {
    dev_id: String,
    port: u8,
    confirmed: bool,
    payload_raw: String,
}

struct AuthHeader(String);

impl FromRequest<'_, '_> for AuthHeader {
    type Error = ();

    fn from_request(request: &Request) -> Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("authorization")
            .map(|s| { AuthHeader(String::from(s)) })
            .ok_or(())
            .into_outcome(
                Status {
                    code: 503,
                    reason: "Authorisation Header required",
                }
            )
    }
}

#[post("/uplink", format = "json", data = "<input>")]
fn uplink(input: Json<UplinkData>, auth: AuthHeader, cfg: State<Configuration>) {
    if auth.0 != cfg.auth_header {
        eprintln!("Invalid auth header value!");
        return;
    }
    let data = base64::decode(&input.payload_raw).expect("Error decoding base64 data in \"payload_raw\"");

    logic::handle_packet(&input.dev_id, input.port, input.counter, &data, &cfg.inner());
}

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
pub async fn downlink(to: String, port: u8, data: &[u8], cfg: &Configuration) -> Result<bool> {
    let client = Client::new();
    let downlink_data = DownlinkData {
        dev_id: to,
        port,
        confirmed: false,
        payload_raw: base64::encode(data),
    };
    let url = format!("https://integrations.thethingsnetwork.org/ttn-eu/api/v2/down/{}/{}", cfg.app_id, cfg.process_id);
    let req = client.post(&url)
        .json(&downlink_data)
        .header("User-Agent", "ttn_http_test/1.0")
        .query(&[("key", &cfg.ttn_key)]);

    Ok(req.send().await?.status().is_success())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    app_id: String,
    dev_id: String,
    process_id: String,
    auth_header: String,
    ttn_key: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            app_id: "any_talk_traveler".to_string(),
            dev_id: "walkietalkie0".to_string(),
            process_id: "ttn_http_test".to_string(),
            auth_header: env!("AUTH_HEADER_VALUE").to_string(),
            ttn_key: env!("TTN_KEY").to_string(),
        }
    }
}

fn main() {
    let cfg: Configuration = confy::load("ttn_http_backend").expect("Error loading config");
    confy::store("ttn_http_backend", &cfg).expect("Error writing config");

    rocket::ignite()
        .manage(cfg)
        .mount("/", routes![uplink])
        .launch();
}
