use log::{debug, error, info, trace, warn};
use social_chat_rust_api::logger_util::{init_log, COMMON_LOG};
use social_chat_rust_api::md5_util::to_md5_str;
use social_chat_rust_api::packs::req_pack::ReqPack;
use social_chat_rust_api::proto_gen_util;
use social_chat_rust_api::protos::auth_success::AuthSuccess;
use std::io::Result;
use std::thread::sleep;
use std::{thread, time};

fn main() -> Result<()> {
    // let mut auth = AuthSuccess::default();
    // auth.user_id = String::from("10000000");
    // // println!("{:?}", auth.encode_to_vec());
    // let mut p = ReqPack::default();
    // p.set_cmd(100).set_sequence(5000).set_data(auth).encode();
    // // .encode();
    // println!("{:?}", p);

    init_log("/Users/simman/Desktop/social-chat-rust-api/logs");

    debug!("this is a debug {}", "message");
    sleep(time::Duration::from_micros(100));
    info!("this is a info message");
    trace!("this is a trace message");
    warn!("this is a warn message");
    error!("this is printed by default");

    for i in 1..1000 {
        error!(target: COMMON_LOG, "this is printed by default, idx: {}", i);
    }

    Ok(())
}
