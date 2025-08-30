#!/usr/bin/env bats

setup() {
  cd "$BATS_TEST_DIRNAME/.."
}

@test "--help exits 0 and mentions Usage" {
  run ./ftime-list.sh --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"Usage: ftime"* ]]
}

@test "--help-short lists -V/--version" {
  run ./ftime-list.sh --help-short
  [ "$status" -eq 0 ]
  [[ "$output" == *"-V/--version"* ]]
}

@test "--version matches VERSION file" {
  ver=$(cat VERSION)
  run bash -lc './ftime-list.sh --version 2>/dev/null'
  [ "$status" -eq 0 ]
  [ "$output" = "$ver" ]
}
