#!/bin/sh

if [ -z "$1" ] ; then
    echo error: no folder specified
    echo usage: `basename $0` \<folder\>
    exit 0
fi

cargo new "$1"
rm "$1/src/main.rs"
mkdir "$1/src/bin/"
cp template.rs "$1/src/bin/part1.rs"
echo 'aoc-utils = { path = "../aoc-utils"}' >> "$1/Cargo.toml"
touch "$1/src/bin/input.txt"