#!/bin/bash

SCRIPT_ROOT="$(realpath "$(cd "$(dirname "$BASH_SOURCE[0]")" && pwd)")"

if [[ "$(realpath "$PWD")" != "$SCRIPT_ROOT" ]]; then
    printf "Incorrect location, try running cd '%s'\n" "$SCRIPT_ROOT" >&2
    exit 1
fi

cargo remove cc --build

template="../../template/multilang"

rm "./build.rs"
rm "./template.s"
rm "./template.c"
rm ./src/*.h
rm ./src/*.hpp
rm ./src/*.c
rm ./src/*.cpp

cp "$template/add-multilang.sh" "./add-multilang.sh"
chmod +x "./add-multilang.sh"
rm "./remove-multilang.sh"
