#!/usr/bin/env bats

setup() {
  cd "$BATS_TEST_DIRNAME/.."
}

@test "unknown option exits 1 and shows error" {
  run bash -lc './ftime-list.sh --bogus 2>&1'
  [ "$status" -eq 1 ]
  [[ "$output" == *"Error: Unknown option '--bogus'"* ]]
  [[ "$output" == *"Usage: ftime"* ]]
}
