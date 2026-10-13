# rig

git の操作を Neovim から扱うための CLI (Rust / 開発中)。

サブコマンド (`log` / `show` / `reflog` / `blame` / `grep` / `refs` / `stash` /
`status`) の定義だけがあり、実装は未着手。未実装のコマンドは実行時に
`anyhow::bail!` で「not implemented yet」を返す。

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
- **依存ポリシー**: `deny.toml` のライセンス許可リストと例外、`Cargo.toml` の
  `[package.metadata.cargo-machete]` の除外リストは、理由をコメントに書いて
  最小限に保つ。例外を外す条件 (未実装コマンドの実装、上流の修正) が来たら
  リストから消す。
