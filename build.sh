#!/usr/bin/env bash

env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release