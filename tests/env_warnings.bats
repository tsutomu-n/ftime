#!/usr/bin/env bats

setup() {
  cd "$BATS_TEST_DIRNAME/.."
}

@test "invalid FTL_TZ warns and is ignored" {
  run bash -lc 'FTL_TZ=Nope ./ftime-list.sh --help-short 2>&1'
  [ "$status" -eq 0 ]
  [[ "$output" == *"Warning: Invalid timezone in FTL_TZ"* ]]
}

@test "FTL_ACTIVE_HOURS must be integer" {
  run bash -lc 'FTL_ACTIVE_HOURS=abc ./ftime-list.sh --help-short 2>&1'
  [ "$status" -eq 0 ]
  [[ "$output" == *"Warning: FTL_ACTIVE_HOURS must be a non-negative integer; using default 4"* ]]
}

@test "FTL_RECENT_HOURS must be integer" {
  run bash -lc 'FTL_RECENT_HOURS=xyz ./ftime-list.sh --help-short 2>&1'
  [ "$status" -eq 0 ]
  [[ "$output" == *"Warning: FTL_RECENT_HOURS must be a non-negative integer; using default 24"* ]]
}

@test "FTL_ACTIVE_HOURS > FTL_RECENT_HOURS warns" {
  run bash -lc 'FTL_ACTIVE_HOURS=48 FTL_RECENT_HOURS=1 ./ftime-list.sh --help-short 2>&1'
  [ "$status" -eq 0 ]
  [[ "$output" == *"Warning: FTL_ACTIVE_HOURS (48) > FTL_RECENT_HOURS (1); colors may look odd"* ]]
}
