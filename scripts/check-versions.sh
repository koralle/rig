#!/usr/bin/env bash
# 版数の整合を検査する。片方だけ更新された状態 (mise と CI で違う版数が使われる、
# ツールチェーンと rust-version がずれる) を検出する。
#
#   rust-toolchain.toml / mise.toml / Cargo.toml  の Rust の版数
#   mise.toml と CI の workflow が pin している cargo ツールの版数
#
# 実行: mise run check-config (または bash scripts/check-versions.sh)
set -euo pipefail

cd "$(dirname "$0")/.."

status=0

ng() {
  echo "NG: $*" >&2
  status=1
}

# --- Rust の版数 ---
toolchain=$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' rust-toolchain.toml)
mise_rust=$(sed -n 's/^rust = "\([^"]*\)"$/\1/p' mise.toml)
# Cargo.toml の rust-version は major.minor だけ書く決まり
msrv=$(sed -n 's/^rust-version = "\([^"]*\)"$/\1/p' Cargo.toml)

if [[ "$toolchain" != "$mise_rust" ]]; then
  ng "rust-toolchain.toml ($toolchain) と mise.toml ($mise_rust) の Rust が違う"
fi
if [[ "$toolchain" != "$msrv".* ]]; then
  ng "Cargo.toml の rust-version ($msrv) が rust-toolchain.toml ($toolchain) と一致しない"
fi

# --- cargo ツール (mise.toml と CI の taiki-e/install-action) ---
# $1: mise.toml のツール名, $2: workflow の tool: に書く名前
check_tool() {
  local mise_key="$1"
  local ci_name="$2"
  local mise_ver ci_vers

  mise_ver=$(sed -n "s/^\"cargo:$mise_key\" = \"\([^\"]*\)\"$/\1/p" mise.toml)
  if [[ -z "$mise_ver" ]]; then
    ng "mise.toml に cargo:$mise_key の版数がない"
    return
  fi

  # grep は該当なしで exit 1 を返すため、set -e / pipefail で落ちないようにする
  ci_vers=$(grep -h -E "^[[:space:]]+tool: *$ci_name@" .github/workflows/*.yaml 2>/dev/null |
    sed -E "s/^[[:space:]]+tool: *$ci_name@(.*)$/\1/" | sort -u || true)
  if [[ -z "$ci_vers" ]]; then
    ng "CI の workflow が $ci_name を pin していない (mise.toml は $mise_ver)"
    return
  fi
  if [[ "$ci_vers" != "$mise_ver" ]]; then
    ng "$mise_key: mise.toml ($mise_ver) と CI ($(echo "$ci_vers" | paste -sd' ' -)) が違う"
  fi
}

check_tool cargo-deny cargo-deny
check_tool cargo-machete cargo-machete
check_tool cargo-nextest nextest

if [[ $status -eq 0 ]]; then
  echo "版数は揃っている (rust $toolchain)"
fi
exit $status
