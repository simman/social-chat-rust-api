use social_chat_rust_api::store_util;

fn main() {
    store_util::init("/Users/simman/Desktop/social-pc/social-chat-rust-api/db_store_dir", "1883838383");

    // store_util::restore();

    // let _ = store_util::get_store_config("path", Some("123123"));

    // store_util::get(None);
}