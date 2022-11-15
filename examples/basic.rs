extern crate aquestalk;

use std::fs;

fn main() {
    let wav = aquestalk::synthe("こんにちは", 100);
    fs::write("test.wav", wav.to_vec()).unwrap();
}
