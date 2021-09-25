#!/bin/sh
export RUST_LOG=debug; ./exonum-domrf run --db-path=./db -c config/config.toml --master-key-pass pass:fd78rw42