# rig

git の操作を Neovim から扱うための CLI (Rust / 開発中)。

実装はこれから。`crates/rig` は CLI の骨組み (`main.rs` / `lib.rs`) だけで、まだ
依存が無い。`crates/rig-gpui` は GPUI Kit で最小のウィンドウを出す GUI の起点で、
こちらは gpui-kit に依存する。CLI のサブコマンドや追加のクレートは必要になった
時点で足す。

## 開発

### 前提

- [mise](https://mise.jdx.dev/) (ツール導入とタスク実行)
- Rust は `rust-toolchain.toml` が固定する。`mise.toml` の `rust` も同じ値に
  してある (mise のグローバル設定が `RUSTUP_TOOLCHAIN` を export していても、
  リポジトリ側の値が勝つようにするため)。版数の整合は `mise run check-config` が
  検査する
- native のビルド依存は不要 (`git2` 0.21 の default features は空なので、libgit2 を
  同梱ソースから `cc` でビルドする)。`https` / `ssh` を有効にすると openssl と
  libssh2 (pkg-config) が要るため、有効化するときは CI にも導入が必要
- `rig-gpui` (GUI) は gpui-kit 経由で GPUI を使うため、Linux では X11 / Wayland /
  fontconfig / xkbcommon の開発パッケージが要る (CI は
  `.github/actions/install-linux-deps` で導入する)。macOS は Xcode Command Line
  Tools、Windows は MSVC + CMake が要る
- 改行コードは `.gitattributes` で LF に統一してある (Windows からの編集でも
  `cargo fmt --all -- --check` が差分を出さないため)

### セットアップ

```sh
mise run setup   # ツール導入 (nextest / machete / deny / lefthook / actionlint / yamllint) + Git フック有効化
```

### タスク

| コマンド | 内容 |
| --- | --- |
| `mise run fmt` | rustfmt で整形する |
| `mise run fmt-check` | 整形されているか確認する |
| `mise run clippy` | Clippy (`-D warnings`) |
| `mise run test` | nextest でテストする |
| `mise run test-doc` | doc テスト |
| `mise run doc` | rustdoc の警告 (壊れたリンクなど) をエラーにする |
| `mise run machete` | 未使用クレートを検出する |
| `mise run deny` | 依存のポリシーチェック (脆弱性 / ライセンス / 取得元) |
| `mise run check-config` | 設定ファイルの検査 (workflow の lint / YAML / ツールの版数) |
| `mise run check` | `fmt-check` + `clippy` + `check-config` (pre-commit と同じ) |
| `mise run ci` | CI と同じ一式を直列で実行する |

`Cargo.toml` の依存を変えたときは `cargo build` などで `Cargo.lock` を更新してから
コミットする (各タスクは `--locked` 付きで走るため、lock が古いと失敗する)。

### GUI (`crates/rig-gpui`)

GPUI (Zed の UI フレームワーク) を [GPUI Kit](https://gpui-kit.com/) 経由で使う。
GPUI 本体は gpui-kit が `gpui-pre-*` のスナップショットとして固定して `gpui_kit::*`
から再エクスポートするため、依存は gpui-kit だけにしてある (gpui を別途入れると
GPUI が二重に入る)。

```sh
cargo run -p rig-gpui   # ウィンドウを開く (既定のビルド対象は CLI なので -p が要る)
```

ウィンドウを出すには X11 / Wayland または macOS / Windows のデスクトップセッションが
要る (ヘッドレスでは起動しない)。画面なしで見るのは headless な smoke test
(`cargo nextest run -p rig-gpui`) で、起動 → 描画 → ボタン押下 → 状態変化までを
テスト用プラットフォーム上で確認する。

## ガードレール

- **workspace + lints**: ルート `Cargo.toml` が workspace。`[workspace.lints]` で
  `unsafe_code = "forbid"`、`rust_2024_compatibility`、Clippy の `unwrap_used` /
  `expect_used` / `panic` / `todo` / `unimplemented` / `dbg_macro` / `unreachable` /
  `unwrap_in_result` / `print_stdout` などを `deny` している (各クレートは
  `[lints] workspace = true` で継承)。`missing_docs` だけは公開 API を持つ
  ライブラリの crate root (`lib.rs`) で `deny` する。未実装は `todo!()` ではなく
  `anyhow::bail!` でエラーを返す。
- **出力の制限**: stdout に書けるのは `main.rs` (バイナリ) だけ。ライブラリ側は
  `clippy::print_stdout` で禁止している (Neovim と stdio でやり取りするため)。
- **テストの緩和** (`clippy.toml`): テストコードに限って `unwrap` / `expect` /
  `panic` を許可する。外部の `git` 起動 (`std::process::Command::new`) と
  `std::process::exit` は禁止 (git 操作は libgit2 経由、終了コードは `main` に集約)。
- **設定の検査** (`mise run check-config`): actionlint (workflow の構文 / 式)、
  yamllint (`.yamllint` の作法)、`scripts/check-versions.sh` (Rust と cargo
  ツールの版数が `rust-toolchain.toml` / `mise.toml` / `Cargo.toml` / CI で
  揃っているか)。
- **ビルド設定**: `.cargo/config.toml` で rustc の警告もエラーにする。release は
  `lto = "thin"` / `codegen-units = 1` / `strip` / `overflow-checks`。
- **Git フック** (`lefthook.yaml`): pre-commit で `mise run check` と `machete`、
  pre-push で `test` / `test-doc` / `doc` / `deny`。`--no-verify` で回避しない。
- **CI** (`.github/workflows/ci.yaml`): 変更ファイルに応じて format / clippy /
  doc / config / deny / machete / test / platforms を実行し、`ci-gate` で結果を
  集約する。`platforms` は macOS / Windows の matrix でビルドとテストを見る。
  `test` は CI 用の nextest プロファイル (`.config/nextest.toml`: fail-fast なし /
  リトライ / ハング検出) を使う。ツールの版数は `mise.toml` を唯一の出所にして、
  CI の pin とのずれは `check-config` が検出する。依存の更新は dependabot
  (週次・グループ化・major は保留)。
- **定期チェック** (`.github/workflows/scheduled-checks.yaml`): 依存が変わらなくても
  増えるもの (新規 advisory / yanked) を週次で検出するため `cargo deny` を実行する。
- **依存ポリシー**: 依存は実装で必要になった時点でルート `Cargo.toml` の
  `[workspace.dependencies]` に追加する。追加したら `deny.toml` の冒頭にある
  既知の注意 (git2 / nvim-rs) を確認し、machete の除外や deny の例外を使う
  場合は理由をコメントに書いて最小限に保ち、解消したら消す。gpui-kit (GUI) を
  入れた時点で `deny.toml` が大きくなっている (build script の許可リスト /
  重複バージョンの skip / MPL-2.0 のクレート例外 / 上流由来の advisory)。
  どれも理由コメント付きで、解消したら消す。

## ライセンス

MIT (詳細は [LICENSE](LICENSE))。依存クレートのライセンスは MIT と互換なものだけを
`deny.toml` の allow リストで許可している (コピーレフトはクレート単位の例外にする)。
