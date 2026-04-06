function test-rust() {
    cargo embed -p test-with-rust-only
}

function test-rust-asm() {
    cargo embed -p test-with-rust-and-asm
}

function test-rust-assembled() {
    cargo embed -p test-with-rust-and-assembled-asm
}

function test-rust-c() {
    cargo embed -p test-with-rust-and-c
}
