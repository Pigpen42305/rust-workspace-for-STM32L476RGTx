[string]$__binary_crate_dirname__ = 'binaries'
[string]$__library_crate_dirname__ = 'libraries'

function test-current-location {
    if ($PWD.Path -ne $PSScriptRoot) {
        throw "Incorrect location, try running Set-Location -Path '$PSScriptRoot'"
    }
}

function test-rust-toolchain {
    $tmp = $env:RUSTUP_LOG
    $env:RUSTUP_LOG = 'WARN'
    try {
        $null = rustup.exe --version
        $null = cargo.exe --version
    }
    catch {
        throw "'$HOME\.cargo\bin' Was not found on PATH or is not installed.`nPlease install and run rustup from the rust foundation website."
    }
    finally {
        $env:RUSTUP_LOG = $tmp
        Remove-Variable -Name "tmp"
    }
}

function New-Crate {
    [CmdletBinding()]
    param (
        [string]$Name,
        [string[]]$Depends,
        [string]$type
    )
    test-current-location
    test-rust-toolchain

    if ($type -ieq 'lib') {
        $_name = $__library_crate_dirname__
        $_source = 'lib'
    } elseif ($type -ieq 'bin') {
        $_name = $__binary_crate_dirname__
        $_source = 'bin'
    } else {
        throw "Bad type: $type"
    }

    $target = Join-Path -Path ".\$_name" -ChildPath $Name

    if (Test-Path -Path $target) {
        throw "$target already exists!"
    } else {
        cargo.exe new $target "--$_source"
        $manifest = "$target\Cargo.toml"

        # Add workplace dependencies
        [string[]]$lines = Get-Content -Path ".\Cargo.toml"

        $start = $lines.IndexOf('[workspace.dependencies]') + 1
        $end = $lines.Length
        for ($i = $start; $i -lt $lines.Length; $i++) {
            if ($lines[$i] -match '^\s*\[.+\]\s*$') {
                $end = $i
                break
            }
        }

        [string[]]$data = Get-Content -Path "$manifest"


        for ($i = $start; $i -lt $end; $i++) {
            $data += $lines[$i]
        }
        Set-Content -Path "$manifest" -Value $data
        Remove-Item -Path "$target\src\**"
        Copy-Item -Path ".\template\$_source\**" -Destination "$target\src"
        Copy-Item -Path ".\template\multilang\Add-Multilang.ps1" -Destination "$target\Add-Multilang.ps1"
    }



    Push-Location -Path $target
    try {
        foreach ($dep in $Depends) {
            if (
                Test-Path -Path (
                    $cargofile = Join-Path -Path "..\..\$__library_crate_dirname__" -ChildPath $dep -AdditionalChildPath 'Cargo.toml'
                )
            ) {
                $folder = Split-Path -Path $cargofile -Parent
                cargo.exe add --path $folder
            }
            else {
                Write-Error -Message "$dep could not be resolved!"
            }
        }
    }
    finally {
        Pop-Location
    }
}

function Remove-Crate {
    [CmdletBinding()]
    param (
        [string]$Name,
        [string]$type
    )
    test-current-location

    if ($type -ieq 'lib') {
        $_name = $__library_crate_dirname__
    } elseif ($type -ieq 'bin') {
        $_name = $__binary_crate_dirname__
    } else {
        throw "Bad type: $type"
    }

    $target = Join-Path -Path ".\$_name" -ChildPath $Name

    
    if (Test-Path -Path $target) {
        Remove-Item -Path $target -Recurse -Force
    } else {
        throw "$target does not exist!"
    }
    
    [string]$members = (($data = Get-Content -Path ".\Cargo.toml") -ilike "members = *")[0]

    $remove = "$_name/$(Split-Path -Path $target -Leaf)"

    # Extract list contents
    $list = $members -replace '^\s*members\s*=\s*\[|\]\s*$', ''

    # Split elements and remove quotes
    $items = $list -split '\s*,\s*' | ForEach-Object {
        $_.Trim("'`"")
    }

    # Remove target
    $items = $items | Where-Object { $_ -ne $remove }

    # Rebuild the string
    $new_members = "members = [" + ($items | ForEach-Object { "`"$_`"" } -join ', ') + "]"

    $data[$data.IndexOf($members)] = $new_members
    Set-Content -Path ".\Cargo.toml" -Value $data
}

function New-LibCrate {
    [CmdletBinding()]
    param (
        [string]$Name,
        [string[]]$Depends
    )
    New-Crate -Name $Name -Depends $Depends -type 'lib'
}

function Remove-LibCrate {
    [CmdletBinding()]
    param (
        [string]$Name
    )
    Remove-Crate -Name $Name -type 'lib'
}

function New-BinCrate {
    [CmdletBinding()]
    param (
        [string]$Name,
        [string[]]$Depends
    )
    New-Crate -Name $Name -Depends $Depends -type 'bin'
}

function Remove-BinCrate {
    param (
        [string]$Name
    )
    Remove-Crate -Name $Name -type 'bin'
}

function Connect-Probe {
    test-current-location

    [string[]]$probes = @(probe-rs list)

    if ($probes.Count -eq 1) {
        throw "$probes"
    }

    $probe = $probes[1]
    Write-Output -InputObject "Selected: $probe"

    try {
        if ($probe -is [char]) {
            throw "Got char"
        }
        $parts = $probe.Split(' -- ')[1].Split(' ')[0].Split(':')
    } catch {
        throw "No probes found"
    }

    $usb_vid = $parts[0]
    $usb_pid = $parts[1]
    $serial = $parts[2]

    $path = ".\Embed.toml"
    if (-not (Test-Path -Path $path)) {
        New-Item -Path $path -ItemType File
    }
    Set-Content -Path $path -Value (
        '[default.general]',
        'chip = "STM32L476RGTx"',
        '',
        '[default.probe]',
        "usb_vid = `"$usb_vid`"",
        "usb_pid = `"$usb_pid`"",
        "serial = `"$serial`"",
        "protocol = `"Swd`"`n"
    )
}

function Test-Workspace {
    [CmdletBinding()]
    param ()

    test-current-location
    test-rust-toolchain

    $installed = (cargo.exe install --list) -like "probe-rs-tools*"

    if ($installed.Count -eq 0) {
        cargo.exe install probe-rs-tools --locked
    }

    try { Connect-Probe } catch { Write-Host -Object "No probes found" -ForegroundColor DarkRed }

    if ((rustup.exe target list --installed) -notcontains 'thumbv7em-none-eabihf') {
        rustup.exe target add thumbv7em-none-eabihf
    }

    if (-not (Get-Command arm-none-eabi-gcc.exe -ErrorAction SilentlyContinue)) {
        Write-Warning "arm-none-eabi-gcc.exe was not found on PATH. Rust and ARMASM remain available, but multilang functionality requires this compiler."
    }

    if (-not (Test-Path $__binary_crate_dirname__)) {
        New-Item $__binary_crate_dirname__ -ItemType Directory
    }
    if (-not (Test-Path $__library_crate_dirname__)) {
        New-Item $__library_crate_dirname__ -ItemType Directory
    }
}
