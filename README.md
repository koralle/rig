# rig

git の操作を Neovim から扱うための CLI (Rust / 開発中)。

実装はこれから。ソースは `crates/rig` の骨組み (`main.rs` / `lib.rs`) だけで、
依存もまだ 1 つも無い。CLI のサブコマンドや追加のクレートは必要になった
時点で足す。

## 開発

### 前提

- [mise](https://mise.jdx.dev/) (ツール導入とタスク実行)
- Rust は `rust-toolchain.toml` が固定する (現在 1.99.0)。`mise.toml` の
  `rust` も同じ値にしてある (mise のグローバル設定が `RUSTUP_TOOLCHAIN` を
  export していても、リポジトリ側の値が勝つようにするため)
- native のビルド依存は不要 (libgit2 は vendored で同梱、`cc` でビルドされる)

### セットアップ

```sh
mise run setup   # ツール導入 (nextest / machete / deny / lefthook) + Git フック有効化
```

### タスク

| コマンド | 内容 |
| --- | --- |
| `mise run fmt` | rustfmt で整形する |
| `mise run fmt-check` | 整形されているか確認する |
| `mise run clippy` | Clippy (`-D warnings`) |
| `mise run test` | nextest でテストする |
| `mise run test-doc` | doc テスト |
| `mise run machete` | 未使用クレートを検出する |
| `mise run deny` | 依存のポリシーチェック (脆弱性 / ライセンス / 取得元) |
| `mise run check` | `fmt-check` + `clippy` (pre-commit と同じ) |
| `mise run ci` | CI と同じ一式を直列で実行する |

## ガードレール

- **workspace + lints**: ルート `Cargo.toml` が workspace。`[workspace.lints]` で
  `unsafe_op_in_unsafe_fn` と Clippy の `unwrap_used` / `expect_used` / `panic` /
  `todo` / `unimplemented` / `dbg_macro` を `deny` している (各クレートは
  `[lints] workspace = true` で継承)。未実装は `todo!()` ではなく
  `anyhow::bail!` でエラーを返す。
- **Git フック** (`lefthook.yaml`): pre-commit で `mise run check` と `machete`、
  pre-push で `test` と `deny`。`--no-verify` で回避しない。
- **CI** (`.github/workflows/ci.yaml`): 変更ファイルに応じて format / clippy /
  deny / machete / test を実行し、`ci-gate` で結果を集約する。ツールの版数は
  `mise.toml` と揃える。依存の更新は dependabot (週次・グループ化・major は保留)。
- **依存ポリシー**: 依存は実装で必要になった時点でルート `Cargo.toml` の
  `[workspace.dependencies]` に追加する。追加したら `deny.toml` の冒頭にある
  既知の注意 (git2 / nvim-rs) を確認し、machete の除外や deny の例外を使う
  場合は理由をコメントに書いて最小限に保ち、解消したら消す。
