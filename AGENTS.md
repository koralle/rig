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

- ツールチェーンは `rust-toolchain.toml`。`mise.toml` の `rust` と必ず揃える
  (mise のグローバル設定が `RUSTUP_TOOLCHAIN` を export していてもリポジトリ側の
  値が勝つようにしてある)。cargo ツールの版数も `mise.toml` を唯一の出所にし、
  CI の pin は `mise run check-config` が突き合わせる。
- 変更を入れる前に `mise run check` (fmt-check / clippy / 設定ファイルの検査)、
  push する前に `mise run test` / `mise run test-doc` / `mise run doc` /
  `mise run deny`。これらは lefthook が自動で実行する。フックは `--no-verify`
  で回避しない。
- ローカルで CI と同じ検証をするときは `mise run ci`。
- cargo を伴うタスクは `--locked` 付きで実行される。依存を足したら `Cargo.lock` も
  同時に更新する (lock が古いままの状態はローカルでも CI でも失敗する)。
- Clippy は `unwrap_used` / `expect_used` / `panic` / `todo` / `unimplemented` /
  `dbg_macro` / `unreachable` / `unwrap_in_result` / `print_stdout` を deny。
  未実装は `todo!()` ではなく `anyhow::bail!` でエラーを返す。テストコードの
  `unwrap` / `expect` / `panic` だけは `clippy.toml` で許可している。
- `unsafe` は `unsafe_code = "forbid"` で書けない。stdout に書けるのは
  `main.rs` (バイナリ) だけで、ライブラリからは書かない (Neovim と stdio で
  やり取りするため)。
- `missing_docs` は workspace lints には入れず、公開 API を持つライブラリの
  crate root (`crates/*/src/lib.rs`) で `#![deny(missing_docs)]` する
  (workspace lints に入れると結合テストの crate root にも docs を要求してしまう)。
- ドキュメントは rustdoc の警告をエラーにして検証する (`mise run doc`)。
  壊れた intra-doc link は `cargo test --doc` では検出されない。
- 依存を足すと決めたら、`cargo machete` と `cargo deny` の設定 (`deny.toml`) も
  一緒に見直す。machete の除外が必要になったら `crates/rig/Cargo.toml` に
  `[package.metadata.cargo-machete]` を理由付きで追加する。deny.toml の skip /
  ignore / 例外 / `[bans.build] allow-build-scripts` を使う場合も、理由を
  コメントに残して最小限に保ち、解消したら消す。
- 依存は `[workspace.dependencies]` に置き、member からは `workspace = true` で
  参照する。使わなくなったエントリは残さない (`[bans.workspace-dependencies]` が
  duplicates / unused を検出する)。
- workflow / YAML を触ったら `mise run check-config` を通す (actionlint /
  yamllint / 版数整合)。CI は `mise run ci` と同等だが、`test` は CI 用プロファイル
  (`.config/nextest.toml` の `ci`) で、`platforms` (macOS / Windows の matrix) は
  CI 固有。
- CI は `.github/workflows/ci.yaml` (変更ファイルに応じた検証) と
  `.github/workflows/scheduled-checks.yaml` (cargo deny の週次実行)。ジョブは
  `mise.toml` のタスクと 1:1 に対応させる (ただし `platforms` は CI 固有)。
