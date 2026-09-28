#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TOOLS="$ROOT/.tools"
if [[ ! -x "$TOOLS/cargo/bin/cargo" && -x "$ROOT/../.tools/cargo/bin/cargo" ]]; then
  TOOLS="$ROOT/../.tools"
fi
RUSTUP_HOME="$TOOLS/rustup"
CARGO_HOME="$TOOLS/cargo"
export RUSTUP_HOME CARGO_HOME
export PATH="$CARGO_HOME/bin:$PATH"

SDK="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Library/Android/sdk}}"
NDK="${ANDROID_NDK_HOME:-$SDK/ndk-bundle}"
HOST_TAG="$(find "$NDK/toolchains/llvm/prebuilt" -mindepth 1 -maxdepth 1 -type d -name 'darwin-*' -exec basename {} \; | head -1)"
TOOLCHAIN_BIN="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin"
if [[ -z "$HOST_TAG" || ! -x "$TOOLCHAIN_BIN/clang" ]]; then
  echo "Android NDK clang was not found under $NDK" >&2
  exit 1
fi

rustup target add aarch64-linux-android x86_64-linux-android

build_one() {
  local target="$1"
  local abi="$2"
  local linker="$TOOLCHAIN_BIN/${target}21-clang"
  [[ -x "$linker" ]] || { echo "Missing linker: $linker" >&2; exit 1; }
  local target_key="$(echo "$target" | tr '[:lower:]-' '[:upper:]_')"
  export "CARGO_TARGET_${target_key}_LINKER=$linker"
  local unwind_dir="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/sysroot/usr/lib/$target"
  if [[ ! -f "$unwind_dir/libunwind.a" ]]; then
    unwind_dir="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/sysroot/usr/lib"
  fi
  export RUSTFLAGS="-C link-arg=-L$unwind_dir"
  env "CC_${target}=$linker" "AR_${target}=$TOOLCHAIN_BIN/llvm-ar" \
    cargo build -p wanandroid-core --release --target "$target"
  mkdir -p "$ROOT/android/rust-lib/$abi"
  cp "$ROOT/target/$target/release/libwanandroid_core.so" "$ROOT/android/rust-lib/$abi/libuniffi_wanandroid.so"
}

build_one aarch64-linux-android arm64-v8a
build_one x86_64-linux-android x86_64
printf 'Rust Android cdylibs generated under android/rust-lib/\n'
