#!/bin/bash

SCRIPT_ROOT="$(realpath "$(cd "$(dirname "$BASH_SOURCE[0]")" && pwd)")"

if [[ "$(realpath "$PWD")" != "$SCRIPT_ROOT" ]]; then
    printf "Incorrect location, try running cd '%s'\n" "$SCRIPT_ROOT" >&2
    exit 1
fi

if ! command -v arm-none-eabi-gcc >/dev/null 2>&1; then
    printf "arm-none-eabi-gcc was not found on PATH\n" >&2
    exit 1
fi

cargo add cortex-m cortex-m-rt panic-halt
cargo add cc --build

template="../../template/multilang"

cp "$template/build.rs" "./"
cp "$template/template.s" "./"
cp "$template/template.c" "./"
cp "$template/crust.h" "./src/"
cp "$template/stm32l476xx.h" "./src/"
cp "$template/stm32l476xx_constants.s" "./src/"

cp "$template/remove-multilang.sh" "./remove-multilang.sh"
chmod +x "./remove-multilang.sh"
rm "./add-multilang.sh"
