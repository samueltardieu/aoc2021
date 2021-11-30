#! /bin/sh -e

cargo run --release | egrep "Part [12]:" > result.txt && diff expected.txt result.txt && rm result.txt
