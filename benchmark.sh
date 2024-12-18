#!/usr/bin/bash

cargo build --release && for i in {01..25}; do [ -e "target/release/day$i" ] && time "target/release/day$i"; done
