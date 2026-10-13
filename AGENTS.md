# rig

git の操作を Neovim から扱う CLI (Rust)。ルートは workspace で、実体は
`crates/rig` のバイナリ + ライブラリ。実装はこれから (ソースは骨組みだけで、
依存も未追加)。

Rust を書くとき:

- 所有権・設計・ドメイン制約・コンパイラエラーを考えるときは
  actionbook/rust-skills を使う。
- 具体的な idiomatic Rust の指針は leonardomso/rust-skills を使う。
- リポジトリ固有のルールが常に優先される。
- 一般的なクレート/ライブラリの推奨を機械的に適用しない。

## このリポジトリのルール

- ツールチェーンは `rust-toolchain.toml` (1.99.0)。`mise.toml` の `rust` と
  必ず揃える (mise のグローバル設定が `RUSTUP_TOOLCHAIN` を export していても
  リポジトリ側の値が勝つようにしてある)。
- 変更を入れる前に `mise run check`、push する前に `mise run test` と
  `mise run deny`。これらは lefthook が自動で実行する。フックは
  `--no-verify` で回避しない。
- ローカルで CI と同じ検証をするときは `mise run ci`。
- Clippy は `unwrap_used` / `expect_used` / `panic` / `todo` / `unimplemented` /
  `dbg_macro` を deny。未実装は `todo!()` ではなく `anyhow::bail!` で
  エラーを返す。テストコードでもまず `Result` を返す形を検討する。
- 依存を足すときは `cargo machete` と `cargo deny` の設定
  (`crates/rig/Cargo.toml` の `[package.metadata.cargo-machete]`、`deny.toml`)
  も一緒に見直す。除外・例外は理由をコメントに残し、解消したら消す。
- CI は `.github/workflows/ci.yaml`。ジョブは `mise.toml` のタスクと 1:1 に
  対応させる (ツールの版数も `mise.toml` に合わせる)。
