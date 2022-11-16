extern crate aquestalk;

use std::fs;

fn main() {
    let wav = aquestalk::synthe("これはてすとです", 100).unwrap();
    fs::write("test.wav", wav.to_vec()).unwrap();
}
