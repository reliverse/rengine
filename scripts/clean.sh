#!/usr/bin/env bash
set -euo pipefail

dry_run=0

dir_targets=(
  .cache
  .expo
  .nitro
  .output
  .source
  .tanstack
  .turbo
  android
  dist
  ios
  node_modules
  target
)

file_targets=(
  bun.lock
  '*.tsbuildinfo'
)

print_help() {
  cat <<'EOF'
Usage (from monorepo root): bun clean [--dry-run]

Recursively removes common build artifacts and dependency directories in a monorepo.

Options:
  -d, --dry-run   Print matched paths without deleting
  -h, --help      Show this help
EOF
}

while (($#)); do
  case "$1" in
    -d|--dry-run) dry_run=1 ;;
    -h|--help)
      print_help
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 2
      ;;
  esac
  shift
done

find_expr=()

append_or_patterns() {
  local first=1
  local pattern

  for pattern in "$@"; do
    if (( first )); then
      first=0
    else
      find_expr+=(-o)
    fi

    find_expr+=(-name "$pattern")
  done
}

build_find_expr() {
  find_expr=(
    .
    -mindepth 1
    \(
      \(
        -type d
        \(
      )

  append_or_patterns "${dir_targets[@]}"

  find_expr+=(
        \)
        -prune
      \)
      -o
      \(
        -type f
        \(
      )

  append_or_patterns "${file_targets[@]}"

  find_expr+=(
        \)
      \)
    \)
  )
}

build_find_expr

if (( dry_run )); then
  find "${find_expr[@]}" -exec printf '%s\n' '{}' +
else
  find "${find_expr[@]}" -exec rm -rf -- '{}' +
fi
