use std::fs;
use std::path::PathBuf;
// use social_chat_rust_api::AuthReq;
use prost::Message;
use regex::Regex;
use social_chat_rust_api::packs::req_pack::ReqPack;
use social_chat_rust_api::proto_gen_util;
use social_chat_rust_api::protos::auth_success::AuthSuccess;
use std::io::Result;
use std::ops::Index;

fn main() -> Result<()> {
    let mut auth = AuthSuccess::default();
    auth.user_id = String::from("10000000");

    // println!("{:?}", auth.encode_to_vec());

    let mut p = ReqPack::default();
    p.set_cmd(100).set_sequence(5000).set_data(auth).encode();
    // .encode();

    println!("{:?}", p);

    // let aq = AuthReq::default();

    // let mut config = prost_build::Config::new();
    // config.out_dir("src/protos");
    // config.btree_map(&["."]);
    // // config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    // config.compile_protos(
    //     &["src/protos/C2S_Auth.protos", "src/protos/C2S_Key.protos"],
    //     &["src/protos"],
    // )?;

    // let dir = fs::read_dir("/Users/simman/Desktop/social-chat-rust-api/protos").unwrap();
    // for f in dir.enumerate() {
    //     println!(
    //         "\"src/protos/{}\",",
    //         f.1.unwrap().file_name().to_str().unwrap()
    //     );
    // }

    // proto_gen_util::gen_proto_code("/Users/simman/Desktop/social-chat-rust-api/protos", "/Users/simman/Desktop/social-chat-rust-api/protos");

    // proto_gen_util::gen_mod("/Users/simman/Desktop/social-chat-rust-api/src/protos");

    // let path = "/Users/simman/Desktop/social-chat-rust-api/protos/S2C_SessionUnReadCount.protos";

    // let p = PathBuf::from("").join("").to_str().unwrap();

    // println!("{:?}", p);

    // let xPathBuf::new().join(test_str);

    Ok(())
}
