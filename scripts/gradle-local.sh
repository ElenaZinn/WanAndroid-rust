#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export GRADLE_USER_HOME="${GRADLE_USER_HOME:-$ROOT/.tools/gradle-home}"
exec "$ROOT/.tools/gradle/gradle-8.7/bin/gradle" "$@"
