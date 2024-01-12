#!/bin/bash
rm README.md
for file in src/bin/*
do
    filename=$(basename $file)
    cargo run --bin "${filename%.*}"
done
