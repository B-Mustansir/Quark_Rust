#!/bin/bash

# Clone the repository if it doesn't exist
if [ ! -d "guessing-game-rust" ]; then
    git clone https://github.com/B-Mustansir/Quark_Rust.git
fi

# Change into the repository directory
cd guessing-game-rust

# Update the repository to make sure we have the latest version
git pull origin master

# Build the Rust code
cargo build --release

# Run the program
./target/release/task2
