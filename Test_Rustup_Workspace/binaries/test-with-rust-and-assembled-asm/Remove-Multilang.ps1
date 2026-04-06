if ((Resolve-Path $PWD).Path -ne (Resolve-Path $PSScriptRoot).Path) {
    throw "$PWD != $PSScriptRoot"
}

cargo remove cc --build

$template = "../../template/multilang"

Copy-Item -Path "$template/Add-Multilang.ps1" -Destination "$PWD/Add-Multilang.ps1"
Remove-Item -Path "build.rs","template.s","template.c"
Remove-Item -Path "src/*.h","src/*.hpp","src/*.c","src/*.cpp"

Remove-Item -Path "$PSScriptRoot/Remove-Multilang.ps1"
