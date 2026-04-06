#!/bin/bash

__binary_crate_dirname__="binaries"
__library_crate_dirname__="libraries"

function check-current-location {
    local script_root="$(realpath "$(cd "$(dirname "$BASH_SOURCE[0]")" && pwd)")"
    local cwd="$(realpath "$PWD")"
    if [[ "$cwd" == "$script_root" ]]; then
        return 0
    fi
    echo "Incorrect location, try running cd '$script_root'" >&2
    return 1
}

function new-crate {
    local name="$1"
    local type="$2"
    shift 2
    local depends=("$@")

    if ! check-current-location; then
        return 1
    fi

    local _name
    local _source

    case "${type,,}" in
        lib)
            _name="$__library_crate_dirname__"
            _source="lib"
            ;;
        bin)
            _name="$__binary_crate_dirname__"
            _source="bin"
            ;;
        *)
            echo "Bad type: $type" >&2
            return 1
            ;;
    esac

    local target="./$_name/$name"

    if [[ -d "$target" ]]; then
        echo "$target already exists!" >&2
        return 1
    fi
    cargo new "$target" --$_source
    local manifest="$target/Cargo.toml"
    
    local workspace_manifest="./Cargo.toml"
    local start_line

    start_line="$(grep -n '^\[workspace\.dependencies\]$' "$workspace_manifest" | cut -d: -f1)"

    awk -v start="$start_line" '
        NR <= start { next }
        /^\[.*\]$/ { exit }
        { print }
    ' "$workspace_manifest" >> "$manifest"

    rm -rf "$target/src"/*
    cp -r "./template/$_source"/. "$target/src/"

    pushd "$target" > /dev/null || return 1

    for dep in "${depends[@]}"; do
        local cargofile="../../$__library_crate_dirname__/$dep/Cargo.toml"
        
        if [[ -e "$cargofile" ]]; then
            local folder="$(dirname "$cargofile")"
            cargo add --path "$folder"
        else
            echo "$dep could not be resolved!" >&2
        fi
    done

    popd > /dev/null || return 1
}

function remove-crate {
    local name="$1"
    local type="$2"

    if ! check-current-location; then
        return 1
    fi

    local _name
    local _source

    case "${type,,}" in
        lib)
            _name="$__library_crate_dirname__"
            _source="lib"
            ;;
        bin)
            _name="$__binary_crate_dirname__"
            _source="bin"
            ;;
        *)
            echo "Bad type: $type" >&2
            return 1
            ;;
    esac

    local target="./$_name/$name"

    if [[ -d "$target" ]]; then
        rm -rf "$target"
    else
        echo "$target does not exist!" >&2
        return 1
    fi

    local cargo_toml="./Cargo.toml"
    local remove="$_name/$(basename "$target")"

    local members="$(grep -i '^members[[:space:]]*=' "$cargo_toml" | head -n 1)"

    local list="$(printf '%s\n' "$members" | sed -E 's/^[[:space:]]*members[[:space:]]*=[[:space:]]*\[//; s/\][[:space:]]*$//')"
    
    local items=()

    IFS=',' read -r -a raw_items <<< "$list"
    for item in "${raw_items[@]}"; do
        item="${item#"${item%%[![:space:]]*}"}"
        item="${item%"${item##*[![:space:]]}"}"
        item="${item%\"}"
        item="${item#\"}"
        item="${item%\'}"
        item="${item#\'}"
    
        if [[ "$item" != "$remove" && -n "$item" ]]; then
            items+=("$item")
        fi
    done

    local new_members='members = ['
    local first=1
    for item in "${items[@]}"; do
        if (( first )); then
            first=0
        else
            new_members+=', '
        fi
        new_members+="\"$item\""
    done
    new_members+=']'

    awk -v new_line="$new_members" '
        /^[[:space:]]*members[[:space:]]*=/ { print new_line; next }
        { print }
    ' "$cargo_toml" > "$cargo_toml.tmp" && mv "$cargo_toml.tmp" "$cargo_toml"
}

function new-libcrate {
    local name="$1"
    shift 1
    new-crate "$name" "lib" "$@"
}

function remove-libcrate {
    remove-crate "$1" "lib"
}

function new-bincrate {
    local name="$1"
    shift 1
    new-crate "$name" "bin" "$@"
}

function remove-bincrate {
    remove-crate "$1" "bin"
}

function connect-probe {
    if ! check-current-location; then
        return 1
    fi

    local probes="$(probe-rs list)" || {
        echo "probe-rs is missing" >&2;
        return 1
    }

    mapfile -t probe_lines <<< "$probes"

    if (( ${#probe_lines[@]} == 1 )); then
        echo "${probe_lines[0]}" >&2
        return 1
    fi

    local probe="${probe_lines[1]}"
    echo "Selected: $probe"

    if [[ -z "$probe" || ${#probe} -eq 1 || "$probe" != *" -- "* ]]; then
        return 1
    fi

    local probe_token="${probe#* -- }"
    probe_token="${probe_token%% *}"

    local parts
    IFS=':' read -r -a parts <<< "$probe_token" || true

    if (( ${#parts[@]} < 2 )); then
        return 1
    fi

    local usb_vid="${parts[0]}"
    local usb_pid="${parts[1]}"
    local serial="${parts[2]}"

    local path="./Embed.toml"

    cat > "$path" <<EOF
[default.general]
chip = "STM32L476RGTx"
connect_under_reset = true

[default.probe]
usb_vid = "$usb_vid"
usb_pid = "$usb_pid"
serial = "$serial"
protocol = "Swd"
speed = 1800

[default.reset]
enabled = true
halt_afterwards = true
EOF
}

function test-workspace {
    if ! check-current-location; then
        return 1
    fi

    if ! python3 --version >/dev/null 2>&1; then
        echo "python3 was not found!" >&2
        return 1
    fi

    local pyversion="$(python3 -c "import sys; print(sys.version_info[1])")"

    if (( pyversion < 10 )); then
        echo "python3.$pyversion.xx is less than python3.10.0, and is not supported!" >&2
        return 1
    fi

    if ! python3 "./python_helper.py"; then
        return 1
    fi

    local tmp="$RUSTUP_LOG"
    RUSTUP_LOG="WARN"
    
    if ! rustup --version >/dev/null 2>&1; then
        echo "rustup is missing!" >&2
        RUSTUP_LOG="$tmp"
        unset tmp
        return 1
    fi

    if ! cargo --version >/dev/null 2>&1; then
        echo "cargo is missing!" >&2
        return 1
    fi

    local result="true"
    while IFS= read -r line; do
        case "$line" in
            probe-rs-tools*)
                result="false"
                break
                ;;
            *)
                continue
                ;;
        esac
    done < <(cargo install --list)

    if $result; then
        cargo install probe-rs-tools --locked
    fi

    unset result

    connect-probe || echo "No probes found" >&2

    local path

    if ! arm-none-eabi-gcc --version >/dev/null 2>&1; then
        echo "arm-none-eabi-gcc is not installed!" >&2
    fi

    [ ! -d "$__binary_crate_dirname__" ] && mkdir "$__binary_crate_dirname__"
    [ ! -d "$__library_crate_dirname__" ] && mkdir "$__library_crate_dirname__"
}
