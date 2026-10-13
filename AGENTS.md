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
- 変更を入れる前に `mise run check`、push する前に `mise run test` /
  `mise run test-doc` / `mise run doc` / `mise run deny`。これらは lefthook が
  自動で実行する。フックは `--no-verify` で回避しない。
- ローカルで CI と同じ検証をするときは `mise run ci`。
- cargo を伴うタスクは `--locked` 付きで実行される。依存を足したら `Cargo.lock` も
  同時に更新する (lock が古いままの状態はローカルでも CI でも失敗する)。
- Clippy は `unwrap_used` / `expect_used` / `panic` / `todo` / `unimplemented` /
  `dbg_macro` を deny。未実装は `todo!()` ではなく `anyhow::bail!` で
  エラーを返す。テストコードでもまず `Result` を返す形を検討する。
- ドキュメントは rustdoc の警告をエラーにして検証する (`mise run doc`)。
  壊れた intra-doc link は `cargo test --doc` では検出されない。
- 依存を足すと決めたら、`cargo machete` と `cargo deny` の設定 (`deny.toml`) も
  一緒に見直す。machete の除外が必要になったら `crates/rig/Cargo.toml` に
  `[package.metadata.cargo-machete]` を理由付きで追加する。deny.toml の skip /
  ignore / 例外や build script を許可する設定 (`[bans.build] allow-build-scripts`)
  を使う場合も、理由をコメントに残して最小限に保ち、解消したら消す。
- CI は `.github/workflows/ci.yaml` (変更ファイルに応じた検証) と
  `.github/workflows/scheduled-checks.yaml` (cargo deny の週次実行)。ジョブは
  `mise.toml` のタスクと 1:1 に対応させる (ツールの版数も `mise.toml` に合わせる)。
