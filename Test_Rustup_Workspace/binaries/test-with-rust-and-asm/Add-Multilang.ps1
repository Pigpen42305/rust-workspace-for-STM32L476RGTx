if ((Resolve-Path $PWD).Path -ne (Resolve-Path $PSScriptRoot).Path) {
    throw "$PWD != $PSScriptRoot"
}

if (-not (Get-Command arm-none-eabi-gcc.exe -ErrorAction SilentlyContinue)) {
    throw "arm-none-eabi-gcc.exe was not found on PATH"
}

cargo add cortex-m cortex-m-rt panic-halt
cargo add cc --build

$template = "../../template/multilang"

Copy-Item -Path "$template/build.rs","$template/template.s","$template/template.c","$template/Remove-Multilang.ps1" -Destination $PWD
Copy-Item -Path "$template/crust.h","$template/stm32l476xx.h","$template/stm32l476xx_constants.s" -Destination "./src/"

Remove-Item -Path "$PSScriptRoot/Add-Multilang.ps1"
