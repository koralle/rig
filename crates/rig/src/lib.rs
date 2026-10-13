//! `cargo test --doc` を通すために lib ターゲットだけ先に置いてある。
//!
//! `missing_docs` は workspace lints ではなくここで deny している。workspace lints に
//! 入れると結合テスト (`tests/*.rs` は別の crate root) にも crate ドキュメントを
//! 要求してしまうため、公開 API を持つライブラリの crate root だけで有効にする。
#![deny(missing_docs)]
