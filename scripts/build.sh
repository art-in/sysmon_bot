#!/bin/bash

# builds service

# go to project root
cd $(dirname $(realpath "$0")) && cd ..

# build
cargo build --release