# aquestalk-rs

This is for aquestalk1 rust wrapper.

## 読み上げに使用する際

aquestalkを使ってDiscord読み上げbotなどを作成する場合aquestalkに問い合わせして、サーバー用ライセンスの購入が必須です。

## Installation

```toml
[dependencies]
aquestalk = "0.1.2"
```

## features

`aquestalk1` - Using aquestalk version 1.

## Usage

### Setup
※This is describe about linux.

1. Please install aquestalk1 at [here](https://www.a-quest.com/download.html)

2. Go to `aqtk1-lnx-eva` directory, choose `lib32` or `lib64` and choose which you like one.

3. move `libAquesTalk.so` file to `/usr/lib`.

All that!

### Code

```rust
fn main() {
    let wav = aquestalk::synthe_aquestalk1("こんにちは").unwrap();
    std::fs::write("test.wav", wav.to_vec()).unwrap();
}
```
