#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TOOLS="$ROOT/../.tools"
RUSTUP_HOME="$TOOLS/rustup"
CARGO_HOME="$TOOLS/cargo"
export RUSTUP_HOME CARGO_HOME
export PATH="$CARGO_HOME/bin:$PATH"
OUT="$ROOT/android/app/src/main/java"
mkdir -p "$OUT"

CLI_ROOT="$TOOLS/uniffi-cli"
if [[ ! -x "$CLI_ROOT/target/release/uniffi-cli-wanandroid" ]]; then
  TMP="$(mktemp -d)"
  trap 'rm -rf "$TMP"' EXIT
  mkdir -p "$TMP/src"
  cat > "$TMP/Cargo.toml" <<'EOF'
[package]
name = "uniffi-cli-wanandroid"
version = "0.1.0"
edition = "2021"
[dependencies]
uniffi = { version = "0.28.3", features = ["cli"] }
EOF
  cat > "$TMP/src/main.rs" <<'EOF'
fn main() { uniffi::uniffi_bindgen_main(); }
EOF
  cargo build --manifest-path "$TMP/Cargo.toml" --release --target-dir "$CLI_ROOT/target"
fi

rm -rf "$OUT/uniffi/wanandroid"
"$CLI_ROOT/target/release/uniffi-cli-wanandroid" generate \
  --language kotlin \
  --out-dir "$OUT" \
  --crate wanandroid_core \
  "$ROOT/rust/wanandroid-core/src/wanandroid.udl"
