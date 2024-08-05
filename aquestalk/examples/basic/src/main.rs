use aquestalk::synthe_aquestalk1;

use std::fs;

fn main() {
    let wav = synthe_aquestalk1("こんにちは", 100).unwrap();
    fs::write("hello.wav", wav).unwrap();
}