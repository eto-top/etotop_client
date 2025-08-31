ordinary_crates := "-p rust_lib_etotop_client"

build +ARGS="":
   (cd flutter; just build {{ARGS}})

check-ordinary +ARGS="":
    cargo check {{ordinary_crates}} {{ARGS}} --all-features --tests --bins

lint-ordinary +ARGS="":
    cargo fmt {{ordinary_crates}} -- --check
    cargo clippy {{ordinary_crates}} {{ARGS}} --all-features --tests --bins -- -Dwarnings

dart-format-check-app:
    ( cd flutter; dart format --set-exit-if-changed --output=none  $(find ./lib -type f -name "*.dart" -not -path "./lib/src/rust/*") )

lint-app +ARGS="": maybe-gen dart-format-check-app
    ( cd flutter; flutter analyze {{ARGS}} )

fix-dart: maybe-gen
    ( cd flutter && dart format $(find ./lib -type f -name "*.dart" -not -path "./lib/src/rust/*") && dart fix --apply && flutter analyze )

maybe-gen:
    just "flutter/maybe-gen"

gen:
    just "flutter/gen"

gen-web:
    just "flutter/gen-web"

fix: fix-dart fix-rust

fix-rust:
    cargo clippy --fix --allow-dirty --allow-staged {{ordinary_crates}} --all-features --tests --bins
    cargo fmt --all

run +ARGS="":
    just flutter/run {{ARGS}}

run-web:
    just "flutter/run-web"

check: check-ordinary
lint: lint-ordinary lint-app

install-cargo-bins:
    just flutter/install-cargo-bins

clean:
    just "flutter/clean"

regen:
    just "flutter/regen"