extern crate amiigen;

use std::env;
fn main() {
    let args : Vec<String> = env::args().collect();
    if let (Some(id), Some(tag_uid), Some(save_to)) = (args.get(1), args.get(2), args.get(3)) {
        let good_id : [u8; 8] = amiigen::decode_hex(id.as_str()).expect("please enter valid id").try_into().expect("please enter valid id");
        let good_uid = amiigen::decode_hex(tag_uid.as_str()).expect("please enter valid tag uid");
        let amiibo = amiigen::gen_amiibo(good_id, good_uid.as_slice()).expect("invalid amiibo data");
        std::fs::write(save_to, amiibo).expect("failed to save");

    }
}
