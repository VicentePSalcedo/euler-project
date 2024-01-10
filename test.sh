#!/bin/bash
rm answer.txt
for file in src/bin/*
do
    filename=$(basename $file)
    cargo run --bin "${filename%.*}"
done
