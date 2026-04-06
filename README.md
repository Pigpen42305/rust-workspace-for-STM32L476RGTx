# Tests to Complete

## Windows

### Windows Base

#### Windows Base Environment

- [ ] `rustup` **is not** installed
- [ ] `arm-none-eabi` **is not** installed

#### Windows Base Tests

- [ ] `CargoHelper.ps1` should import
- [ ] `Test-Workspace` should ***fail***
- [ ] `New-BinCrate` and `New-LibCrate` should ***fail***

### Windows and `rustup`

#### Windows and `rustup` Environment

- [ ] `rustup` **is** installed
- [ ] `arm-none-eabi` **is not** installed

#### Windows and `rustup` Tests

- [ ] `CargoHelper.ps1` should import
- [ ] `Test-Workspace` should *emit warnings*, but ***succeed anyway***
- [ ] `New-BinCrate` and `New-LibCrate` should ***succeed***
- [ ] `Remove-BinCrate` and `Remove-LibCrate` should ***succeed***
- [ ] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [ ] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust
- [ ] `Add-Multilang.ps1` should ***fail***

### Windows and `rustup` and `arm-none-eabi`

#### Windows and `rustup` and `arm-none-eabi` Environment

- [x] `rustup` **is** installed
- [x] `arm-none-eabi` **is** installed

#### Windows and `rustup` and `arm-none-eabi` Tests

- [x] `CargoHelper.ps1` should import
- [x] `Test-Workspace` should ***succeed***
- [x] `New-BinCrate` and `New-LibCrate` should ***succeed***
- [x] `Remove-BinCrate` and `Remove-LibCrate` should ***succeed***
- [x] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [x] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust using Rust's `global_asm!` macro
- [x] `Add-Multilang.ps1` should ***succeed***
- [x] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [x] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust using Rust's C ffi
- [x] A C program should compile and run successfully
  - The C program should be called by Rust using Rust's C ffi
- [x] `Remove-Multilang.ps1` should ***succeed***

## Linux

### Linux Base

#### Linux Base Environment

- [ ] `rustup` **is not** installed
- [ ] `arm-none-eabi` **is not** installed

#### Linux Base Tests

- [ ] `cargo-helper.sh` should import
- [ ] `test-workspace` should ***fail***
- [ ] `new-bincrate` and `new-libcrate` should ***fail***

### Linux and `rustup`

#### Linux and `rustup` Environment

- [ ] `rustup` **is** installed
- [ ] `arm-none-eabi` **is not** installed

#### Linux and `rustup` Tests

- [ ] `cargo-helper.sh` should import
- [ ] `test-workspace` should *emit warnings*, but ***succeed anyway***
- [ ] `new-bincrate` and `new-libcrate` should ***succeed***
- [ ] `remove-bincrate` and `remove-libcrate` should ***succeed***
- [ ] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [ ] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust
- [ ] `add-multilang.sh` should ***fail***

### Linux and `rustup` and `arm-none-eabi`

#### Linux and `rustup` and `arm-none-eabi` Environment

- [ ] `rustup` **is** installed
- [ ] `arm-none-eabi` **is** installed

#### Linux and `rustup` and `arm-none-eabi` Tests

- [ ] `cargo-helper.sh` should import
- [ ] `test-workspace` should ***succeed***
- [ ] `new-bincrate` and `new-libcrate` should ***succeed***
- [ ] `remove-bincrate` and `remove-libcrate` should ***succeed***
- [ ] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [ ] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust using Rust's `global_asm!` macro
- [ ] `add-multilang.sh` should ***succeed***
- [ ] A Rust program should compile and run successfully
  - Rust dependencies should be added automatically
- [ ] An Arm-Assembly program should assemble and run successfully
  - The Assembly program should be called by Rust using Rust's C ffi
- [ ] A C program should compile and run successfully
  - The C program should be called by Rust using Rust's C ffi
  - The C++ program should be called by Rust using Rust's C ffi
- [ ] `remove-multilang.sh` should ***succeed***
