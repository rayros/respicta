coverage: coverage-build coverage-lcov coverage-html

coverage-build:
  mkdir -p coverage
  rm -rf target_coverage
  CARGO_TARGET_DIR="target_coverage" CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target_coverage/coverage/%p-%m.profraw' cargo test

coverage-lcov:
  grcov target_coverage/coverage \
  --llvm \
  --binary-path ./target_coverage/debug/deps/ \
  -s . \
  --branch \
  --ignore-not-existing \
  --excl-start '^(pub(\((crate|super)\))? )?mod tests' \
  --excl-stop '^}' \
  --ignore="target_coverage/*" \
  --ignore="*/tests/*" \
  --ignore="src/main.rs" \
  -t lcov \
  -o coverage/lcov.info

coverage-html:
  rm -rf coverage/html
  grcov target_coverage/coverage \
  --llvm \
  --binary-path ./target_coverage/debug/deps/ \
  -s . \
  --branch \
  --ignore-not-existing \
  --excl-start '^(pub(\((crate|super)\))? )?mod tests' \
  --excl-stop '^}' \
  --ignore="target_coverage/*" \
  --ignore="*/tests/*" \
  --ignore="src/main.rs" \
  -t html \
  -o coverage/html