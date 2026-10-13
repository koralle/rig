//! `rig` の CLI エントリポイント。
//!
//! stdout に書いてよいのはこのバイナリだけにする (ライブラリ側では
//! `clippy::print_stdout` が deny されている。ここで明示的に許可する)。
#![allow(clippy::print_stdout)]

fn main() {}
