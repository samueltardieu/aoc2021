#! /bin/sh -e

cargo run --release -- --all > result.txt && diff expected.txt result.txt && rm result.txt
