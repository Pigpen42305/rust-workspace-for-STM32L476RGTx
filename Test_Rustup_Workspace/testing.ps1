function test-rust() {
    cargo.exe embed -p test-with-rust-only
}

function test-rust-asm() {
    cargo.exe embed -p test-with-rust-and-asm
}

function test-rust-assembled() {
    cargo.exe embed -p test-with-rust-and-assembled-asm
}

function test-rust-c() {
    cargo.exe embed -p test-with-rust-and-c
}
