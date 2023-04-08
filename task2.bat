@echo off

set REPO_NAME = Quark_Rust
set REPO_URL = https://github.com/B-Mustansir/Quark_Rust.git

if not exist %REPO_NAME% git clone %REPO_URL%
cd %REPO_NAME%
git pull origin master
cargo run --release --bin task2.rs
