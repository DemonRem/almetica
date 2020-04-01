#![warn(clippy::all)]
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use warp::Filter;

#[derive(Serialize)]
struct ServerCharactersInfo {
    id: i32,
    char_count: u32,
}

#[derive(Serialize)]
struct AuthResponse {
    last_connected_server_id: i32, // 1
    chars_per_server: Vec<ServerCharactersInfo>,
    account_bits: String, // ??? Possible vlaue: 0x041F000D or 0x00000000?

    #[serde(rename = "result-message")]
    result_message: String, // OK

    #[serde(rename = "result-code")]
    result_code: i32, // 200

    access_level: i32,           // Normal user = 1
    user_permission: i32,        // Normal user = 0
    game_account_name: String,   // Always "TERA"
    master_account_name: String, // We will use a UUID here, so that LOGIN and GAME server don't need to expose their indexes for synchronization.
    ticket: String, // Can be any string that is ASCII printable. Use some kind of signature so that LOGIN and GAME server don't need a connection to each other.
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // The TERA client NEEDS to have the region endings (.uk / .de etc.) at the end or else it will not start!

    // GET /server/list.uk
    let server = warp::get().and(warp::path!("server" / "list.uk")).map(|| {
        r###"<serverlist>
<server>
<id>1</id>
<ip>127.0.0.1</ip>
<port>10001</port>
<category sort="1">Almetica</category>
<name raw_name="Almetica"> Almetica </name>
<crowdness sort="1">None</crowdness>
<open sort="1">Recommended</open>
<permission_mask>0x00000000</permission_mask>
<server_stat>0x00000000</server_stat>
<popup> This server isn't up yet! </popup>
<language>en</language>
</server>
</serverlist>"###
    });

    // GET /auth
    let auth = warp::post()
        .and(warp::path("auth"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::form())
        .map(|_simple_map: HashMap<String, String>| {
            // TODO proper auth handling
            let resp = AuthResponse {
                last_connected_server_id: 4001,
                chars_per_server: vec![],
                account_bits: "0x00000000".to_string(),
                result_message: "OK".to_string(),
                result_code: 200,
                access_level: 1,
                user_permission: 0,
                game_account_name: "TERA".to_string(),
                master_account_name: "cb3c75d4-66a6-4506-a549-c8ae53fbafd8".to_string(),
                ticket: "OScGKtmr3sngb418rFnHEDWMTrYSbHa280jveZtCeG7T7pXv7HOScGKtmr3sngb418rFnHEDWMTrYSbHa280jveZtCeG7T7pXv7H".to_string(),
            };

            warp::reply::json(&resp)
        });

    let log = warp::log("almetica::login");
    let routes = server.or(auth).with(log);

    // TODO read from configuration
    let listen_addr_string = "127.0.0.1:8080";
    let listen_addr: SocketAddr = listen_addr_string.parse().expect("Unable to parse listen address");
    warp::serve(routes).run(listen_addr).await;
}
