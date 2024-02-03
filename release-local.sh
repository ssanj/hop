#!/bin/bash

cargo build --release
cp target/release/hop ~/bin/
hop -V
